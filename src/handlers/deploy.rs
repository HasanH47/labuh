use axum::{
    extract::{Extension, Path, State},
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio_stream::StreamExt;

use crate::error::Result;
use crate::middleware::auth::CurrentUser;
use crate::models::ProjectResponse;
use crate::services::{ContainerService, DeployService};

/// Deploy a project
async fn deploy_project(
    State(deploy_service): State<Arc<DeployService>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>> {
    let project = deploy_service.deploy(&id, &current_user.id).await?;
    Ok(Json(project.into()))
}

/// Stop a deployed project
async fn stop_project(
    State(deploy_service): State<Arc<DeployService>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>> {
    let project = deploy_service.stop(&id, &current_user.id).await?;
    Ok(Json(project.into()))
}

/// Restart a deployed project
async fn restart_project(
    State(deploy_service): State<Arc<DeployService>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>> {
    let project = deploy_service.restart(&id, &current_user.id).await?;
    Ok(Json(project.into()))
}

pub fn deploy_routes(deploy_service: Arc<DeployService>) -> Router {
    Router::new()
        .route("/{id}/deploy", post(deploy_project))
        .route("/{id}/stop", post(stop_project))
        .route("/{id}/restart", post(restart_project))
        .with_state(deploy_service)
}

// ============ SSE Streaming Endpoints ============

/// Stream container logs via SSE
pub async fn stream_logs(
    State(container_service): State<Arc<ContainerService>>,
    Path(id): Path<String>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        loop {
            match container_service.get_container_logs(&id, 50).await {
                Ok(logs) => {
                    for line in logs {
                        yield Ok(Event::default().data(line));
                    }
                }
                Err(e) => {
                    yield Ok(Event::default().data(format!("Error: {}", e)));
                    break;
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

/// Stream container stats via SSE
pub async fn stream_stats(
    State(container_service): State<Arc<ContainerService>>,
    Path(id): Path<String>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        loop {
            match container_service.get_container_stats(&id).await {
                Ok(stats) => {
                    if let Ok(json) = serde_json::to_string(&stats) {
                        yield Ok(Event::default().data(json));
                    }
                }
                Err(e) => {
                    yield Ok(Event::default().data(format!("{{\"error\": \"{}\"}}", e)));
                    break;
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

pub fn streaming_routes(container_service: Arc<ContainerService>) -> Router {
    Router::new()
        .route("/{id}/logs/stream", get(stream_logs))
        .route("/{id}/stats/stream", get(stream_stats))
        .with_state(container_service)
}
