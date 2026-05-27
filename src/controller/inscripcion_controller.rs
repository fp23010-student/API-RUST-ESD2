use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::inscripcion::{CreateInscripcionDto, UpdateInscripcionDto};
use crate::service::inscripcion_service as inscripcion_service;

pub async fn get_all(
    State(pool): State<PgPool>,
) -> (StatusCode, Json<serde_json::Value>) {
    match inscripcion_service::get_all(&pool).await {
        Ok(inscripciones) => (StatusCode::OK, Json(serde_json::json!(inscripciones))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(e.to_string()))),
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    match inscripcion_service::get_by_id(&pool, id).await {
        Ok(Some(inscripcion)) => (StatusCode::OK, Json(serde_json::json!(inscripcion))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Inscripcion no encontrada"))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(e.to_string()))),
    }
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(dto): Json<CreateInscripcionDto>,
) -> (StatusCode, Json<serde_json::Value>) {
    match inscripcion_service::create(&pool, &dto).await {
        Ok(inscripcion) => (StatusCode::CREATED, Json(serde_json::json!(inscripcion))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(e.to_string()))),
    }
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateInscripcionDto>,
) -> (StatusCode, Json<serde_json::Value>) {
    match inscripcion_service::update(&pool, id, &dto).await {
        Ok(Some(inscripcion)) => (StatusCode::OK, Json(serde_json::json!(inscripcion))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Inscripcion no encontrada"))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(e.to_string()))),
    }
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    match inscripcion_service::delete(&pool, id).await {
        Ok(rows) if rows > 0 => (StatusCode::OK, Json(serde_json::json!("Inscripcion eliminada correctamente"))),
        Ok(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Inscripcion no encontrada"))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(e.to_string()))),
    }
}