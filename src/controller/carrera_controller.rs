use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::carrera::CarreraInput;
use crate::service::carrera_service;

pub async fn get_all(State(pool): State<PgPool>) -> impl IntoResponse {
    match carrera_service::listar_carreras(&pool).await {
        Ok(carreras) => (StatusCode::OK, Json(serde_json::json!(carreras))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al obtener carreras"))),
    }
}

pub async fn get_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match carrera_service::obtener_carrera(&pool, id).await {
        Ok(Some(carrera)) => (StatusCode::OK, Json(serde_json::json!(carrera))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Carrera no encontrada"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al obtener carrera"))),
    }
}

pub async fn create(State(pool): State<PgPool>, Json(body): Json<CarreraInput>) -> impl IntoResponse {
    match carrera_service::crear_carrera(&pool, body).await {
        Ok(carrera) => (StatusCode::CREATED, Json(serde_json::json!(carrera))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al crear carrera"))),
    }
}

pub async fn update(State(pool): State<PgPool>, Path(id): Path<i32>, Json(body): Json<CarreraInput>) -> impl IntoResponse {
    match carrera_service::actualizar_carrera(&pool, id, body).await {
        Ok(Some(carrera)) => (StatusCode::OK, Json(serde_json::json!(carrera))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Carrera no encontrada"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al actualizar carrera"))),
    }
}

pub async fn delete(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match carrera_service::eliminar_carrera(&pool, id).await {
        Ok(rows) if rows > 0 => (StatusCode::OK, Json(serde_json::json!("Carrera eliminada"))),
        Ok(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Carrera no encontrada"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al eliminar carrera"))),
    }
}