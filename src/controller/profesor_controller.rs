use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::profesor::CreateProfesor;
use crate::service::profesor_service;

pub async fn get_all(State(pool): State<PgPool>) -> impl IntoResponse {
    match profesor_service::listar_profesores(&pool).await {
        Ok(profesores) => (StatusCode::OK, Json(serde_json::json!(profesores))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al obtener profesores"))),
    }
}

pub async fn get_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match profesor_service::obtener_profesor(&pool, id).await {
        Ok(Some(profesor)) => (StatusCode::OK, Json(serde_json::json!(profesor))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Profesor no encontrado"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al obtener profesor"))),
    }
}

pub async fn create(State(pool): State<PgPool>, Json(body): Json<CreateProfesor>) -> impl IntoResponse {
    match profesor_service::crear_profesor(&pool, body).await {
        Ok(profesor) => (StatusCode::CREATED, Json(serde_json::json!(profesor))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al crear profesor"))),
    }
}

pub async fn update(State(pool): State<PgPool>, Path(id): Path<i32>, Json(body): Json<CreateProfesor>) -> impl IntoResponse {
    match profesor_service::actualizar_profesor(&pool, id, body).await {
        Ok(Some(profesor)) => (StatusCode::OK, Json(serde_json::json!(profesor))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Profesor no encontrado"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al actualizar profesor"))),
    }
}

pub async fn delete(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match profesor_service::eliminar_profesor(&pool, id).await {
        Ok(rows) if rows > 0 => (StatusCode::OK, Json(serde_json::json!("Profesor eliminado correctamente"))),
        Ok(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!("Profesor no encontrado"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!("Error al eliminar profesor"))),
    }
}
