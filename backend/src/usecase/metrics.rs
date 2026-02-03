use crate::domain::metrics_repository::{
    HistoricalContainerMetrics, HistoricalNodeMetrics, MetricsRepository,
};
use crate::error::Result;
use std::sync::Arc;

pub struct MetricsUsecase {
    repo: Arc<dyn MetricsRepository>,
}

impl MetricsUsecase {
    pub fn new(repo: Arc<dyn MetricsRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_node_metrics(
        &self,
        last_hours: Option<i32>,
    ) -> Result<Vec<HistoricalNodeMetrics>> {
        let hours = last_hours.unwrap_or(24);
        self.repo.get_node_metrics(hours).await
    }

    pub async fn get_container_metrics(
        &self,
        container_id: &str,
        last_hours: Option<i32>,
    ) -> Result<Vec<HistoricalContainerMetrics>> {
        let hours = last_hours.unwrap_or(24);
        self.repo.get_container_metrics(container_id, hours).await
    }
}
