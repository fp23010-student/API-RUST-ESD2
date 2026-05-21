use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use sqlx::PgPool;
use serde_json::{json, Value};

use crate::models::profesor::Profesor;
use crate::service::profesor_service;

pub async fn get_profesores(
    State(pool): State<PgPool>,
) -> Result<Json<Value>, StatusCode> {

    match profesor_service::listar_profesores(&pool).await {
        Ok(profesores) => Ok(Json(json!(profesores))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_profesor_por_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<Value>, StatusCode> {

    match profesor_service::buscar_profesor(&pool, id).await {
        Ok(profesor) => Ok(Json(json!(profesor))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn crear_profesor(
    State(pool): State<PgPool>,
    Json(profesor): Json<Profesor>,
) -> Result<Json<Value>, StatusCode> {

    match profesor_service::crear_profesor(&pool, profesor).await {
        Ok(_) => Ok(Json(json!({
            "mensaje": "Profesor creado"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn actualizar_profesor(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(profesor): Json<Profesor>,
) -> Result<Json<Value>, StatusCode> {

    match profesor_service::actualizar_profesor(&pool, id, profesor).await {
        Ok(_) => Ok(Json(json!({
            "mensaje": "Profesor actualizado"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_profesor(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<Value>, StatusCode> {

    match profesor_service::eliminar_profesor(&pool, id).await {
        Ok(_) => Ok(Json(json!({
            "mensaje": "Profesor eliminado"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
