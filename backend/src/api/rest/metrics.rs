use crate::domain::metrics_repository::{HistoricalContainerMetrics, HistoricalNodeMetrics};
use crate::error::Result;
use crate::usecase::metrics::MetricsUsecase;
use axum::{
    extract::{Path, Query, State},
    Json, Router,
};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct MetricsQuery {
    pub last_hours: Option<i32>,
}

async fn get_node_metrics(
    State(usecase): State<Arc<MetricsUsecase>>,
    Query(query): Query<MetricsQuery>,
) -> Result<Json<Vec<HistoricalNodeMetrics>>> {
    let metrics = usecase.get_node_metrics(query.last_hours).await?;
    Ok(Json(metrics))
}

async fn get_container_metrics(
    State(usecase): State<Arc<MetricsUsecase>>,
    Path((_stack_id, container_id)): Path<(String, String)>,
    Query(query): Query<MetricsQuery>,
) -> Result<Json<Vec<HistoricalContainerMetrics>>> {
    let metrics = usecase
        .get_container_metrics(&container_id, query.last_hours)
        .await?;
    Ok(Json(metrics))
}

pub fn metrics_routes(usecase: Arc<MetricsUsecase>) -> Router {
    Router::new()
        .route("/nodes/metrics", axum::routing::get(get_node_metrics))
        .route(
            "/stacks/{stack_id}/containers/{container_id}/metrics",
            axum::routing::get(get_container_metrics),
        )
        .with_state(usecase)
}
