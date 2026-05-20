use sqlx::PgPool;
use crate::models::carrera::{Carrera, CarreraInput};

pub async fn get_all_carreras(pool: &PgPool) -> Result<Vec<Carrera>, sqlx::Error> {
    sqlx::query_as::<_, Carrera>(
        "SELECT id_carrera, nombre_carrera, facultad FROM Carreras"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_carrera_by_id(pool: &PgPool, id: i32) -> Result<Option<Carrera>, sqlx::Error> {
    sqlx::query_as::<_, Carrera>(
        "SELECT id_carrera, nombre_carrera, facultad FROM Carreras WHERE id_carrera = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_carrera(pool: &PgPool, input: &CarreraInput) -> Result<Carrera, sqlx::Error> {
    sqlx::query_as::<_, Carrera>(
        "INSERT INTO Carreras (nombre_carrera, facultad)
         VALUES ($1, $2)
         RETURNING id_carrera, nombre_carrera, facultad"
    )
    .bind(&input.nombre_carrera)
    .bind(&input.facultad)
    .fetch_one(pool)
    .await
}

pub async fn update_carrera(pool: &PgPool, id: i32, input: &CarreraInput) -> Result<Option<Carrera>, sqlx::Error> {
    sqlx::query_as::<_, Carrera>(
        "UPDATE Carreras SET nombre_carrera = $1, facultad = $2
         WHERE id_carrera = $3
         RETURNING id_carrera, nombre_carrera, facultad"
    )
    .bind(&input.nombre_carrera)
    .bind(&input.facultad)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_carrera(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM Carreras WHERE id_carrera = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}