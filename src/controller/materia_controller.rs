use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::materia::MateriaInput;
use crate::service::materia_service;

pub async fn get_all(State(pool): State<PgPool>) -> impl IntoResponse {
    match materia_service::listar_materias(&pool).await {
        Ok(materias) => (StatusCode::OK, Json(serde_json::json!(materias))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al obtener materias"))),
    }
}

pub async fn get_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match materia_service::obtener_materia(&pool, id).await {
        Ok(Some(materia)) => (StatusCode::OK, Json(serde_json::json!(materia))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Materia no encontrada"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al obtener materia"))),
    }
}

pub async fn create(State(pool): State<PgPool>, Json(body): Json<MateriaInput>) -> impl IntoResponse {
    match materia_service::crear_materia(&pool, body).await {
        Ok(materia) => (StatusCode::CREATED, Json(serde_json::json!(materia))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al crear materia"))),
    }
}

pub async fn update(State(pool): State<PgPool>, Path(id): Path<i32>, Json(body): Json<MateriaInput>) -> impl IntoResponse {
    match materia_service::actualizar_materia(&pool, id, body).await {
        Ok(Some(materia)) => (StatusCode::OK, Json(serde_json::json!(materia))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Materia no encontrada"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al actualizar materia"))),
    }
}

pub async fn delete(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match materia_service::eliminar_materia(&pool, id).await {
        Ok(rows) if rows > 0 => (StatusCode::OK, Json(serde_json::json!("Materia eliminada"))),
        Ok(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Materia no encontrada"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al eliminar materia"))),
    }
}
