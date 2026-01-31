use std::sync::Arc;
use chrono::{Utc, Duration};
use crate::domain::models::resource::{ContainerResource, ResourceMetric};
use crate::domain::resource_repository::ResourceRepository;
use crate::domain::stack_repository::StackRepository;
use crate::error::Result;

pub struct ResourceUsecase {
    repo: Arc<dyn ResourceRepository>,
    stack_repo: Arc<dyn StackRepository>,
}

impl ResourceUsecase {
    pub fn new(repo: Arc<dyn ResourceRepository>, stack_repo: Arc<dyn StackRepository>) -> Self {
        Self { repo, stack_repo }
    }

    pub async fn update_limits(
        &self,
        stack_id: &str,
        service_name: &str,
        user_id: &str,
        cpu: Option<f64>,
        memory: Option<i64>,
    ) -> Result<()> {
        // Verify ownership
        let _stack = self.stack_repo.find_by_id(stack_id, user_id).await?;

        self.repo.update_resource_limits(stack_id, service_name, cpu, memory).await?;
        Ok(())
    }

    pub async fn get_limits(&self, stack_id: &str, user_id: &str) -> Result<Vec<ContainerResource>> {
        // Verify ownership
        let _stack = self.stack_repo.find_by_id(stack_id, user_id).await?;

        self.repo.list_resource_limits_for_stack(stack_id).await
    }

    pub async fn get_metrics(
        &self,
        stack_id: &str,
        user_id: &str,
        range: &str,
    ) -> Result<Vec<ResourceMetric>> {
        // Verify ownership
        let _stack = self.stack_repo.find_by_id(stack_id, user_id).await?;

        let duration = match range {
            "1h" => Duration::hours(1),
            "6h" => Duration::hours(6),
            "24h" => Duration::hours(24),
            "7d" => Duration::days(7),
            "30d" => Duration::days(30),
            _ => Duration::hours(1),
        };

        let since = (Utc::now() - duration).to_rfc3339();
        self.repo.get_metrics_for_stack(stack_id, &since).await
    }

    pub async fn prune_old_metrics(&self) -> Result<()> {
        let older_than = (Utc::now() - Duration::days(30)).to_rfc3339();
        self.repo.prune_metrics(&older_than).await
    }
}
