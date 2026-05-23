use sqlx::PgPool;
use crate::models::materia::{Materia, MateriaInput};

pub async fn get_all_materias(pool: &PgPool) -> Result<Vec<Materia>, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        "SELECT id_materia, nombre_materia, uv, id_profesor FROM Materias"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_materia_by_id(pool: &PgPool, id: i32) -> Result<Option<Materia>, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        "SELECT id_materia, nombre_materia, uv, id_profesor FROM Materias WHERE id_materia = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_materia(pool: &PgPool, input: &MateriaInput) -> Result<Materia, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        "INSERT INTO Materias (nombre_materia, uv, id_profesor)
         VALUES ($1, $2, $3)
         RETURNING id_materia, nombre_materia, uv, id_profesor"
    )
    .bind(&input.nombre_materia)
    .bind(input.uv)
    .bind(input.id_profesor)
    .fetch_one(pool)
    .await
}

pub async fn update_materia(pool: &PgPool, id: i32, input: &MateriaInput) -> Result<Option<Materia>, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        "UPDATE Materias SET nombre_materia = $1, uv = $2, id_profesor = $3
         WHERE id_materia = $4
         RETURNING id_materia, nombre_materia, uv, id_profesor"
    )
    .bind(&input.nombre_materia)
    .bind(input.uv)
    .bind(input.id_profesor)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_materia(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM Materias WHERE id_materia = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
