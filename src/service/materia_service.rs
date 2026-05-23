use sqlx::PgPool;
use crate::models::materia::{Materia, MateriaInput};
use crate::repository::materia_repository;

pub async fn listar_materias(
    pool: &PgPool
) -> Result<Vec<Materia>, sqlx::Error> {
    materia_repository::get_all_materias(pool).await
}

pub async fn obtener_materia(
    pool: &PgPool,
    id: i32
    ) -> Result<Option<Materia>, sqlx::Error> {
    materia_repository::get_materia_by_id(pool, id).await
}

pub async fn crear_materia(
    pool: &PgPool,
    input: MateriaInput
    ) -> Result<Materia, sqlx::Error> {
    materia_repository::create_materia(pool, &input).await
}

pub async fn actualizar_materia(
    pool: &PgPool,
    id: i32,
    input: MateriaInput
    ) -> Result<Option<Materia>, sqlx::Error> {
    materia_repository::update_materia(pool, id, &input).await
}

pub async fn eliminar_materia(
    pool: &PgPool,
    id: i32
    ) -> Result<u64, sqlx::Error> {
    materia_repository::delete_materia(pool, id).await
}
