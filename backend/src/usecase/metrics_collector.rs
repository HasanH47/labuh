use chrono::Utc;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

use crate::domain::metrics_repository::{
    HistoricalContainerMetrics, HistoricalNodeMetrics, MetricsRepository,
};
use crate::domain::models::ResourceMetric;
use crate::domain::resource_repository::ResourceRepository;
use crate::domain::runtime::RuntimePort;
use crate::domain::stack_repository::StackRepository;
use crate::domain::system::SystemProvider;

pub struct MetricsCollector {
    stack_repo: Arc<dyn StackRepository>,
    resource_repo: Arc<dyn ResourceRepository>,
    metrics_repo: Arc<dyn MetricsRepository>,
    runtime: Arc<dyn RuntimePort>,
    system_provider: Arc<dyn SystemProvider>,
}

impl MetricsCollector {
    pub fn new(
        stack_repo: Arc<dyn StackRepository>,
        resource_repo: Arc<dyn ResourceRepository>,
        metrics_repo: Arc<dyn MetricsRepository>,
        runtime: Arc<dyn RuntimePort>,
        system_provider: Arc<dyn SystemProvider>,
    ) -> Self {
        Self {
            stack_repo,
            resource_repo,
            metrics_repo,
            runtime,
            system_provider,
        }
    }

    pub async fn start(&self) {
        tracing::info!("Starting metrics collector...");

        loop {
            if let Err(e) = self.collect_metrics().await {
                tracing::error!("Error collecting metrics: {}", e);
            }

            // Collect every minute
            sleep(Duration::from_secs(60)).await;
        }
    }

    async fn collect_metrics(&self) -> crate::error::Result<()> {
        let now = Utc::now().to_rfc3339();

        // 1. Node Metrics
        if let Ok(stats) = self.system_provider.get_stats().await {
            let node_metrics = HistoricalNodeMetrics {
                cpu_percent: stats.load_average.one * 10.0, // Rough estimation
                memory_usage: (stats.memory_total_kb - stats.memory_available_kb) * 1024,
                memory_total: stats.memory_total_kb * 1024,
                disk_usage: stats.disk_total_bytes - stats.disk_available_bytes,
                disk_total: stats.disk_total_bytes,
                timestamp: now.clone(),
            };

            if let Err(e) = self.metrics_repo.save_node_metrics(node_metrics).await {
                tracing::error!("Failed to save node metrics: {}", e);
            }
        }

        let stacks = self.stack_repo.list_all().await?;
        let containers = match self.runtime.list_containers(false).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Failed to list containers for metrics: {}", e);
                return Ok(());
            }
        };

        for stack in stacks {
            let stack_containers: Vec<_> = containers
                .iter()
                .filter(|c| {
                    c.labels
                        .get("labuh.stack.id")
                        .map(|id| id == &stack.id)
                        .unwrap_or(false)
                })
                .collect();

            for container in stack_containers {
                match self.runtime.get_stats(&container.id).await {
                    Ok(stats) => {
                        // Legacy metric storage
                        let metric = ResourceMetric {
                            id: Uuid::new_v4().to_string(),
                            container_id: container.id.clone(),
                            stack_id: stack.id.clone(),
                            cpu_usage: stats.cpu_percent,
                            memory_usage: stats.memory_usage as i64,
                            timestamp: now.clone(),
                        };

                        if let Err(e) = self.resource_repo.save_metric(metric).await {
                            tracing::error!("Failed to save metric: {}", e);
                        }

                        // Historical metric storage
                        let hist_metric = HistoricalContainerMetrics {
                            container_id: container.id.clone(),
                            stack_id: stack.id.clone(),
                            cpu_percent: stats.cpu_percent,
                            memory_usage: stats.memory_usage,
                            memory_limit: stats.memory_limit,
                            timestamp: now.clone(),
                        };

                        if let Err(e) = self.metrics_repo.save_container_metrics(hist_metric).await
                        {
                            tracing::error!("Failed to save historical container metric: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::debug!(
                            "Failed to get stats for container {}: {}",
                            container.id,
                            e
                        );
                    }
                }
            }
        }

        // Also prune old metrics
        if let Err(e) = self
            .resource_repo
            .prune_metrics(&(Utc::now() - chrono::Duration::days(30)).to_rfc3339())
            .await
        {
            tracing::error!("Failed to prune metrics: {}", e);
        }

        Ok(())
    }
}
