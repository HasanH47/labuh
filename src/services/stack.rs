//! Stack service for managing Docker Compose stacks

use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::Result;
use crate::models::Stack;
use crate::services::compose::{parse_compose, service_to_container_request};
use crate::services::ContainerService;

pub struct StackService {
    db: SqlitePool,
    container_service: Arc<ContainerService>,
}

impl StackService {
    pub fn new(db: SqlitePool, container_service: Arc<ContainerService>) -> Self {
        Self { db, container_service }
    }

    /// List all stacks for a user
    pub async fn list_stacks(&self, user_id: &str) -> Result<Vec<Stack>> {
        let stacks = sqlx::query_as::<_, Stack>(
            "SELECT * FROM stacks WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        Ok(stacks)
    }

    /// Get a single stack
    pub async fn get_stack(&self, id: &str, user_id: &str) -> Result<Stack> {
        let stack = sqlx::query_as::<_, Stack>(
            "SELECT * FROM stacks WHERE id = ? AND user_id = ?"
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound("Stack not found".to_string()))?;

        Ok(stack)
    }

    /// Create a new stack from docker-compose.yml
    pub async fn create_stack(&self, name: &str, compose_content: &str, user_id: &str) -> Result<Stack> {
        // Parse the compose file first
        let parsed = parse_compose(compose_content)?;

        // Create stack in database
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO stacks (id, name, user_id, compose_content, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(name)
        .bind(user_id)
        .bind(compose_content)
        .bind("creating")
        .bind(&now)
        .bind(&now)
        .execute(&self.db)
        .await?;

        // Create containers for each service
        for service in &parsed.services {
            let request = service_to_container_request(service, &id, name);

            // Pull image first
            self.container_service.pull_image(&request.image).await?;

            // Create container
            self.container_service.create_container(request).await?;
        }

        // Update stack status to stopped
        sqlx::query("UPDATE stacks SET status = ?, updated_at = ? WHERE id = ?")
            .bind("stopped")
            .bind(&Utc::now().to_rfc3339())
            .bind(&id)
            .execute(&self.db)
            .await?;

        self.get_stack(&id, user_id).await
    }

    /// Get containers belonging to a stack
    pub async fn get_stack_containers(&self, stack_id: &str) -> Result<Vec<crate::services::container::ContainerInfo>> {
        let all_containers = self.container_service.list_containers(true).await?;

        // Filter containers by name prefix (stack_name-service_name)
        let stack = sqlx::query_as::<_, Stack>("SELECT * FROM stacks WHERE id = ?")
            .bind(stack_id)
            .fetch_optional(&self.db)
            .await?;

        if let Some(stack) = stack {
            let prefix = format!("/{}-", stack.name);
            let containers: Vec<_> = all_containers
                .into_iter()
                .filter(|c| c.names.iter().any(|n| n.starts_with(&prefix)))
                .collect();
            return Ok(containers);
        }

        Ok(vec![])
    }

    /// Start all containers in a stack
    pub async fn start_stack(&self, id: &str, user_id: &str) -> Result<()> {
        let stack = self.get_stack(id, user_id).await?;
        let containers = self.get_stack_containers(&stack.id).await?;

        for container in containers {
            if container.state != "running" {
                self.container_service.start_container(&container.id).await?;
            }
        }

        sqlx::query("UPDATE stacks SET status = ?, updated_at = ? WHERE id = ?")
            .bind("running")
            .bind(&Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    /// Stop all containers in a stack
    pub async fn stop_stack(&self, id: &str, user_id: &str) -> Result<()> {
        let stack = self.get_stack(id, user_id).await?;
        let containers = self.get_stack_containers(&stack.id).await?;

        for container in containers {
            if container.state == "running" {
                self.container_service.stop_container(&container.id).await?;
            }
        }

        sqlx::query("UPDATE stacks SET status = ?, updated_at = ? WHERE id = ?")
            .bind("stopped")
            .bind(&Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    /// Remove a stack and all its containers
    pub async fn remove_stack(&self, id: &str, user_id: &str) -> Result<()> {
        let stack = self.get_stack(id, user_id).await?;
        let containers = self.get_stack_containers(&stack.id).await?;

        // Stop and remove all containers
        for container in containers {
            if container.state == "running" {
                self.container_service.stop_container(&container.id).await?;
            }
            self.container_service.remove_container(&container.id, true).await?;
        }

        // Delete stack from database
        sqlx::query("DELETE FROM stacks WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
