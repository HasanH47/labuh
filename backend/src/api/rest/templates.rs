use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use std::sync::Arc;

use crate::domain::models::{Template, TemplateResponse};
use crate::error::Result;
use crate::usecase::template::TemplateUsecase;

pub fn template_routes(usecase: Arc<TemplateUsecase>) -> Router {
    Router::new()
        .route("/", get(list_templates).post(create_template))
        .route("/import", axum::routing::post(import_template))
        .route("/{id}", get(get_template).delete(delete_template))
        .with_state(usecase)
}

async fn list_templates(
    State(usecase): State<Arc<TemplateUsecase>>,
) -> Result<Json<Vec<TemplateResponse>>> {
    let templates = usecase.list_templates().await?;
    Ok(Json(templates))
}

async fn get_template(
    State(usecase): State<Arc<TemplateUsecase>>,
    Path(id): Path<String>,
) -> Result<Json<Template>> {
    let template = usecase.get_template(&id).await?;
    Ok(Json(template))
}

async fn create_template(
    State(usecase): State<Arc<TemplateUsecase>>,
    Json(template): Json<Template>,
) -> Result<Json<()>> {
    usecase.create_template(template).await?;
    Ok(Json(()))
}

#[derive(serde::Deserialize)]
struct ImportTemplateRequest {
    url: String,
}

async fn import_template(
    State(usecase): State<Arc<TemplateUsecase>>,
    Json(payload): Json<ImportTemplateRequest>,
) -> Result<Json<Template>> {
    let template = usecase.import_from_url(&payload.url).await?;
    Ok(Json(template))
}

async fn delete_template(
    State(usecase): State<Arc<TemplateUsecase>>,
    Path(id): Path<String>,
) -> Result<Json<()>> {
    usecase.delete_template(&id).await?;
    Ok(Json(()))
}
