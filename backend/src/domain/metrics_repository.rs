use crate::error::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HistoricalNodeMetrics {
    pub cpu_percent: f64,
    pub memory_usage: u64,
    pub memory_total: u64,
    pub disk_usage: u64,
    pub disk_total: u64,
    pub timestamp: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HistoricalContainerMetrics {
    pub container_id: String,
    pub stack_id: String,
    pub cpu_percent: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub timestamp: String,
}

#[async_trait]
pub trait MetricsRepository: Send + Sync {
    async fn save_node_metrics(&self, metrics: HistoricalNodeMetrics) -> Result<()>;
    async fn save_container_metrics(&self, metrics: HistoricalContainerMetrics) -> Result<()>;
    async fn get_node_metrics(&self, last_hours: i32) -> Result<Vec<HistoricalNodeMetrics>>;
    async fn get_container_metrics(
        &self,
        container_id: &str,
        last_hours: i32,
    ) -> Result<Vec<HistoricalContainerMetrics>>;
}
