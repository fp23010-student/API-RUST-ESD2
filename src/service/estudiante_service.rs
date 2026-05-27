use crate::models::estudiante::{CreateEstudianteDto, Estudiante, UpdateEstudianteDto};
use crate::repository::estudiante_repository;
use sqlx::PgPool;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Estudiante>, sqlx::Error> {
    estudiante_repository::get_all(pool).await
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Estudiante>, sqlx::Error> {
    estudiante_repository::get_by_id(pool, id).await
}

pub async fn create(
    pool: &PgPool,
    dto: CreateEstudianteDto,
) -> Result<Estudiante, sqlx::Error> {
    estudiante_repository::create(pool, dto).await
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    dto: UpdateEstudianteDto,
) -> Result<Option<Estudiante>, sqlx::Error> {
    estudiante_repository::update(pool, id, dto).await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    estudiante_repository::delete(pool, id).await
}