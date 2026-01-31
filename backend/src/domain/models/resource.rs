use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContainerResource {
    pub id: String,
    pub stack_id: String,
    pub service_name: String,
    pub cpu_limit: Option<f64>,
    pub memory_limit: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ResourceMetric {
    pub id: String,
    pub container_id: String,
    pub stack_id: String,
    pub cpu_usage: f64,
    pub memory_usage: i64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResourceRequest {
    pub cpu_limit: Option<f64>,
    pub memory_limit: Option<i64>,
}
