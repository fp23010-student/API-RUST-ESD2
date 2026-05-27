use sqlx::PgPool;

use crate::models::inscripcion::{CreateInscripcionDto, Inscripcion, UpdateInscripcionDto};

pub async fn get_all(pool: &PgPool) -> Result<Vec<Inscripcion>, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        "SELECT id_inscripcion, id_estudiante, id_materia, ciclo, nota_final::FLOAT8
         FROM inscripciones",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Inscripcion>, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        "SELECT id_inscripcion, id_estudiante, id_materia, ciclo, nota_final::FLOAT8
         FROM inscripciones WHERE id_inscripcion = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(pool: &PgPool, dto: &CreateInscripcionDto) -> Result<Inscripcion, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        "INSERT INTO inscripciones (id_estudiante, id_materia, ciclo, nota_final)
         VALUES ($1, $2, $3, $4::NUMERIC)
         RETURNING id_inscripcion, id_estudiante, id_materia, ciclo, nota_final::FLOAT8",
    )
    .bind(dto.id_estudiante)
    .bind(dto.id_materia)
    .bind(&dto.ciclo)
    .bind(dto.nota_final)
    .fetch_one(pool)
    .await
}

pub async fn update(pool: &PgPool, id: i32, dto: &UpdateInscripcionDto) -> Result<Option<Inscripcion>, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        "UPDATE inscripciones
         SET id_estudiante = COALESCE($1, id_estudiante),
             id_materia    = COALESCE($2, id_materia),
             ciclo         = COALESCE($3, ciclo),
             nota_final    = COALESCE($4::NUMERIC, nota_final)
         WHERE id_inscripcion = $5
         RETURNING id_inscripcion, id_estudiante, id_materia, ciclo, nota_final::FLOAT8",
    )
    .bind(dto.id_estudiante)
    .bind(dto.id_materia)
    .bind(dto.ciclo.as_deref())
    .bind(dto.nota_final)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM inscripciones WHERE id_inscripcion = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}