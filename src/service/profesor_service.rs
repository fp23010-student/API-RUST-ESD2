use sqlx::PgPool;
use crate::models::profesor::{Profesor, CreateProfesor};
use crate::repository::profesor_repository;

pub async fn listar_profesores(
    pool: &PgPool,
) -> Result<Vec<Profesor>, sqlx::Error> {
    profesor_repository::obtener_profesores(pool).await
}

pub async fn obtener_profesor(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Profesor>, sqlx::Error> {
    profesor_repository::obtener_profesor_por_id(pool, id).await
}

pub async fn crear_profesor(
    pool: &PgPool,
    input: CreateProfesor,
) -> Result<Profesor, sqlx::Error> {
    profesor_repository::crear_profesor(pool, &input).await
}

pub async fn actualizar_profesor(
    pool: &PgPool,
    id: i32,
    input: CreateProfesor,
) -> Result<Option<Profesor>, sqlx::Error> {
    profesor_repository::actualizar_profesor(pool, id, &input).await
}

pub async fn eliminar_profesor(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {
    profesor_repository::eliminar_profesor(pool, id).await
}
