use crate::models::estudiante::{CreateEstudianteDto, Estudiante, UpdateEstudianteDto};
use sqlx::PgPool;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Estudiante>, sqlx::Error> {
    sqlx::query_as!(
        Estudiante,
        "SELECT id_estudiante, nombre, apellido, carnet, fecha_nacimiento, id_carrera 
         FROM Estudiantes 
         ORDER BY id_estudiante"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Estudiante>, sqlx::Error> {
    sqlx::query_as!(
        Estudiante,
        "SELECT id_estudiante, nombre, apellido, carnet, fecha_nacimiento, id_carrera 
         FROM Estudiantes 
         WHERE id_estudiante = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    dto: CreateEstudianteDto,
) -> Result<Estudiante, sqlx::Error> {
    sqlx::query_as!(
        Estudiante,
        "INSERT INTO Estudiantes (nombre, apellido, carnet, fecha_nacimiento, id_carrera)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id_estudiante, nombre, apellido, carnet, fecha_nacimiento, id_carrera",
        dto.nombre,
        dto.apellido,
        dto.carnet,
        dto.fecha_nacimiento,
        dto.id_carrera
    )
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    dto: UpdateEstudianteDto,
) -> Result<Option<Estudiante>, sqlx::Error> {
    sqlx::query_as!(
        Estudiante,
        "UPDATE Estudiantes SET
            nombre = COALESCE($1, nombre),
            apellido = COALESCE($2, apellido),
            carnet = COALESCE($3, carnet),
            fecha_nacimiento = COALESCE($4, fecha_nacimiento),
            id_carrera = COALESCE($5, id_carrera)
         WHERE id_estudiante = $6
         RETURNING id_estudiante, nombre, apellido, carnet, fecha_nacimiento, id_carrera",
        dto.nombre,
        dto.apellido,
        dto.carnet,
        dto.fecha_nacimiento,
        dto.id_carrera,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM Estudiantes WHERE id_estudiante = $1",
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}