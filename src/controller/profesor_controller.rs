use axum::{
    extract::State,
    Json,
    http::StatusCode,
};

use sqlx::PgPool;

use crate::service::profesor_service;

pub async fn get_profesores(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {

    match profesor_service::listar_profesores(&pool).await {
        Ok(profesores) => Ok(Json(serde_json::json!(profesores))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
