use sqlx::PgPool;
use crate::models::profesor::{Profesor, CreateProfesor};

pub async fn obtener_profesores(
    pool: &PgPool,
) -> Result<Vec<Profesor>, sqlx::Error> {
    sqlx::query_as::<_, Profesor>(
        "SELECT id, nombre, correo, especialidad FROM profesores"
    )
    .fetch_all(pool)
    .await
}

pub async fn obtener_profesor_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Profesor>, sqlx::Error> {
    sqlx::query_as::<_, Profesor>(
        "SELECT id, nombre, correo, especialidad FROM profesores WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn crear_profesor(
    pool: &PgPool,
    input: &CreateProfesor,
) -> Result<Profesor, sqlx::Error> {
    sqlx::query_as::<_, Profesor>(
        r#"
        INSERT INTO profesores (nombre, correo, especialidad)
        VALUES ($1, $2, $3)
        RETURNING id, nombre, correo, especialidad
        "#
    )
    .bind(&input.nombre)
    .bind(&input.correo)
    .bind(&input.especialidad)
    .fetch_one(pool)
    .await
}

pub async fn actualizar_profesor(
    pool: &PgPool,
    id: i32,
    input: &CreateProfesor,
) -> Result<Option<Profesor>, sqlx::Error> {
    sqlx::query_as::<_, Profesor>(
        r#"
        UPDATE profesores 
        SET nombre = $1, correo = $2, especialidad = $3
        WHERE id = $4
        RETURNING id, nombre, correo, especialidad
        "#
    )
    .bind(&input.nombre)
    .bind(&input.correo)
    .bind(&input.especialidad)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn eliminar_profesor(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM profesores WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
