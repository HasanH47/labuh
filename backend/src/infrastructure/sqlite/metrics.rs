use crate::domain::metrics_repository::{
    HistoricalContainerMetrics, HistoricalNodeMetrics, MetricsRepository,
};
use crate::error::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteMetricsRepository {
    pool: SqlitePool,
}

impl SqliteMetricsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MetricsRepository for SqliteMetricsRepository {
    async fn save_node_metrics(&self, metrics: HistoricalNodeMetrics) -> Result<()> {
        sqlx::query(
            "INSERT INTO node_metrics (id, cpu_percent, memory_usage, memory_total, disk_usage, disk_total, timestamp)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(metrics.cpu_percent)
        .bind(metrics.memory_usage as i64)
        .bind(metrics.memory_total as i64)
        .bind(metrics.disk_usage as i64)
        .bind(metrics.disk_total as i64)
        .bind(metrics.timestamp)
        .execute(&self.pool)
        .await?;

        // Cleanup: remove older than 7 days
        let _ =
            sqlx::query("DELETE FROM node_metrics WHERE timestamp < datetime('now', '-7 days')")
                .execute(&self.pool)
                .await;

        Ok(())
    }

    async fn save_container_metrics(&self, metrics: HistoricalContainerMetrics) -> Result<()> {
        sqlx::query(
            "INSERT INTO container_metrics (id, container_id, stack_id, cpu_percent, memory_usage, memory_limit, timestamp)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(metrics.container_id)
        .bind(metrics.stack_id)
        .bind(metrics.cpu_percent)
        .bind(metrics.memory_usage as i64)
        .bind(metrics.memory_limit as i64)
        .bind(metrics.timestamp)
        .execute(&self.pool)
        .await?;

        // Cleanup
        let _ = sqlx::query(
            "DELETE FROM container_metrics WHERE timestamp < datetime('now', '-7 days')",
        )
        .execute(&self.pool)
        .await;

        Ok(())
    }

    async fn get_node_metrics(&self, last_hours: i32) -> Result<Vec<HistoricalNodeMetrics>> {
        let rows = sqlx::query_as::<_, HistoricalNodeRow>(
            "SELECT cpu_percent, memory_usage, memory_total, disk_usage, disk_total, timestamp
             FROM node_metrics
             WHERE timestamp > datetime('now', '-' || ? || ' hours')
             ORDER BY timestamp ASC",
        )
        .bind(last_hours)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(HistoricalNodeMetrics::from).collect())
    }

    async fn get_container_metrics(
        &self,
        container_id: &str,
        last_hours: i32,
    ) -> Result<Vec<HistoricalContainerMetrics>> {
        let rows = sqlx::query_as::<_, HistoricalContainerRow>(
            "SELECT container_id, stack_id, cpu_percent, memory_usage, memory_limit, timestamp
             FROM container_metrics
             WHERE container_id = ? AND timestamp > datetime('now', '-' || ? || ' hours')
             ORDER BY timestamp ASC",
        )
        .bind(container_id)
        .bind(last_hours)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(HistoricalContainerMetrics::from)
            .collect())
    }
}

#[derive(sqlx::FromRow)]
struct HistoricalNodeRow {
    cpu_percent: f64,
    memory_usage: i64,
    memory_total: i64,
    disk_usage: i64,
    disk_total: i64,
    timestamp: String,
}

impl From<HistoricalNodeRow> for HistoricalNodeMetrics {
    fn from(row: HistoricalNodeRow) -> Self {
        Self {
            cpu_percent: row.cpu_percent,
            memory_usage: row.memory_usage as u64,
            memory_total: row.memory_total as u64,
            disk_usage: row.disk_usage as u64,
            disk_total: row.disk_total as u64,
            timestamp: row.timestamp,
        }
    }
}

#[derive(sqlx::FromRow)]
struct HistoricalContainerRow {
    container_id: String,
    stack_id: String,
    cpu_percent: f64,
    memory_usage: i64,
    memory_limit: i64,
    timestamp: String,
}

impl From<HistoricalContainerRow> for HistoricalContainerMetrics {
    fn from(row: HistoricalContainerRow) -> Self {
        Self {
            container_id: row.container_id,
            stack_id: row.stack_id,
            cpu_percent: row.cpu_percent,
            memory_usage: row.memory_usage as u64,
            memory_limit: row.memory_limit as u64,
            timestamp: row.timestamp,
        }
    }
}
