use crate::models::estudiante::{CreateEstudianteDto, UpdateEstudianteDto};
use crate::service::estudiante_service;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;

pub async fn get_all(State(pool): State<PgPool>) -> impl IntoResponse {
    match estudiante_service::get_all(&pool).await {
        Ok(estudiantes) => (StatusCode::OK, Json(serde_json::json!(estudiantes))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))),
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match estudiante_service::get_by_id(&pool, id).await {
        Ok(Some(estudiante)) => (StatusCode::OK, Json(serde_json::json!(estudiante))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Estudiante no encontrado"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))),
    }
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(body): Json<CreateEstudianteDto>,
) -> impl IntoResponse {
    match estudiante_service::create(&pool, body).await {
        Ok(estudiante) => (StatusCode::CREATED, Json(serde_json::json!(estudiante))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))),
    }
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateEstudianteDto>,
) -> impl IntoResponse {
    match estudiante_service::update(&pool, id, body).await {
        Ok(Some(estudiante)) => (StatusCode::OK, Json(serde_json::json!(estudiante))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Estudiante no encontrado"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))),
    }
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match estudiante_service::delete(&pool, id).await {
        Ok(true) => (StatusCode::OK, Json(serde_json::json!({"mensaje": "Estudiante eliminado"}))),
        Ok(false) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Estudiante no encontrado"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))),
    }
}