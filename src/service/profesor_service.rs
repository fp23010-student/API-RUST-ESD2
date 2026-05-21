use sqlx::PgPool;

use crate::models::profesor::Profesor;
use crate::repository::profesor_repository;

pub async fn listar_profesores(
    pool: &PgPool,
) -> Result<Vec<Profesor>, sqlx::Error> {

    profesor_repository::obtener_profesores(pool).await
}

pub async fn buscar_profesor(
    pool: &PgPool,
    id: i32,
) -> Result<Profesor, sqlx::Error> {

    profesor_repository::obtener_profesor_por_id(pool, id).await
}

pub async fn crear_profesor(
    pool: &PgPool,
    profesor: Profesor,
) -> Result<(), sqlx::Error> {

    profesor_repository::crear_profesor(pool, profesor).await
}

pub async fn actualizar_profesor(
    pool: &PgPool,
    id: i32,
    profesor: Profesor,
) -> Result<(), sqlx::Error> {

    profesor_repository::actualizar_profesor(pool, id, profesor).await
}

pub async fn eliminar_profesor(
    pool: &PgPool,
    id: i32,
) -> Result<(), sqlx::Error> {

    profesor_repository::eliminar_profesor(pool, id).await
}
