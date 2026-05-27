use sqlx::PgPool;
use crate::models::carrera::{Carrera, CarreraInput};
use crate::repository::carrera_repository;

pub async fn listar_carreras(pool: &PgPool) -> Result<Vec<Carrera>, sqlx::Error> {
    carrera_repository::get_all_carreras(pool).await
}

pub async fn obtener_carrera(pool: &PgPool, id: i32) -> Result<Option<Carrera>, sqlx::Error> {
    carrera_repository::get_carrera_by_id(pool, id).await
}

pub async fn crear_carrera(pool: &PgPool, input: CarreraInput) -> Result<Carrera, sqlx::Error> {
    carrera_repository::create_carrera(pool, &input).await
}

pub async fn actualizar_carrera(pool: &PgPool, id: i32, input: CarreraInput) -> Result<Option<Carrera>, sqlx::Error> {
    carrera_repository::update_carrera(pool, id, &input).await
}

pub async fn eliminar_carrera(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    carrera_repository::delete_carrera(pool, id).await
}