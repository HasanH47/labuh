use axum::{
    extract::{Extension, Path, Query, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::api::middleware::auth::CurrentUser;
use crate::domain::runtime::{ContainerInfo, ContainerStats};
use crate::error::Result;
use crate::usecase::stack::StackUsecase;

#[derive(Deserialize)]
pub struct ListContainersQuery {
    #[serde(default)]
    all: bool,
}

#[derive(Deserialize)]
pub struct LogsQuery {
    #[serde(default = "default_tail")]
    tail: usize,
}

fn default_tail() -> usize {
    100
}

async fn list_containers(
    State(stack_usecase): State<Arc<StackUsecase>>,
    Extension(user): Extension<CurrentUser>,
    Query(query): Query<ListContainersQuery>,
) -> Result<Json<Vec<ContainerInfo>>> {
    let stacks = stack_usecase.list_stacks(&user.id).await?;
    let stack_ids: std::collections::HashSet<String> = stacks.into_iter().map(|s| s.id).collect();

    let all_containers = stack_usecase.runtime().list_containers(query.all).await?;
    let filtered: Vec<ContainerInfo> = all_containers
        .into_iter()
        .filter(|c| {
            c.labels
                .get("labuh.stack.id")
                .map(|id| stack_ids.contains(id))
                .unwrap_or(false)
        })
        .collect();

    Ok(Json(filtered))
}

// Handlers for specific container actions that check ownership
async fn start_container(
    State(stack_usecase): State<Arc<StackUsecase>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    stack_usecase.start_container(&id, &user.id).await?;
    Ok(Json(serde_json::json!({ "status": "started" })))
}

async fn stop_container(
    State(stack_usecase): State<Arc<StackUsecase>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    stack_usecase.stop_container(&id, &user.id).await?;
    Ok(Json(serde_json::json!({ "status": "stopped" })))
}

async fn restart_container(
    State(stack_usecase): State<Arc<StackUsecase>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    stack_usecase.restart_container(&id, &user.id).await?;
    Ok(Json(serde_json::json!({ "status": "restarted" })))
}

async fn remove_container(
    State(stack_usecase): State<Arc<StackUsecase>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    stack_usecase.remove_container(&id, &user.id).await?;
    Ok(Json(serde_json::json!({ "status": "removed" })))
}

async fn get_container_logs(
    State(stack_usecase): State<Arc<StackUsecase>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<String>,
    Query(query): Query<LogsQuery>,
) -> Result<Json<Vec<String>>> {
    let logs = stack_usecase
        .get_container_logs(&id, &user.id, query.tail)
        .await?;
    Ok(Json(logs))
}

async fn get_container_stats(
    State(stack_usecase): State<Arc<StackUsecase>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<ContainerStats>> {
    let stats = stack_usecase.get_container_stats(&id, &user.id).await?;
    Ok(Json(stats))
}

pub fn container_routes(stack_usecase: Arc<StackUsecase>) -> Router {
    Router::new()
        .route("/", get(list_containers))
        .route("/{id}/start", post(start_container))
        .route("/{id}/stop", post(stop_container))
        .route("/{id}/restart", post(restart_container))
        .route("/{id}", delete(remove_container))
        .route("/{id}/logs", get(get_container_logs))
        .route("/{id}/stats", get(get_container_stats))
        .with_state(stack_usecase)
}
