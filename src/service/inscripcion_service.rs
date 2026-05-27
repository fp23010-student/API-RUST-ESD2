use sqlx::PgPool;
use crate::models::inscripcion::{CreateInscripcionDto, Inscripcion, UpdateInscripcionDto};
use crate::repository::inscripcion_repository as inscripcion_repository;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Inscripcion>, sqlx::Error> {
    inscripcion_repository::get_all(pool).await
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Inscripcion>, sqlx::Error> {
    inscripcion_repository::get_by_id(pool, id).await
}

pub async fn create(pool: &PgPool, dto: &CreateInscripcionDto) -> Result<Inscripcion, sqlx::Error> {
    inscripcion_repository::create(pool, dto).await
}

pub async fn update(pool: &PgPool, id: i32, dto: &UpdateInscripcionDto) -> Result<Option<Inscripcion>, sqlx::Error> {
    inscripcion_repository::update(pool, id, dto).await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    inscripcion_repository::delete(pool, id).await
}