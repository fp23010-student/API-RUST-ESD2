use sqlx::{PgPool, query_as, query};

use crate::models::profesor::Profesor;

pub async fn obtener_profesores(
    pool: &PgPool,
) -> Result<Vec<Profesor>, sqlx::Error> {

    let profesores = query_as::<_, Profesor>(
        "SELECT id, nombre, correo, especialidad FROM profesores"
    )
    .fetch_all(pool)
    .await?;

    Ok(profesores)
}

pub async fn obtener_profesor_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Profesor, sqlx::Error> {

    let profesor = query_as::<_, Profesor>(
        "SELECT id, nombre, correo, especialidad FROM profesores WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(profesor)
}

pub async fn crear_profesor(
    pool: &PgPool,
    profesor: Profesor,
) -> Result<(), sqlx::Error> {

    query(
        "INSERT INTO profesores (nombre, correo, especialidad)
        VALUES ($1, $2, $3)"
    )
    .bind(profesor.nombre)
    .bind(profesor.correo)
    .bind(profesor.especialidad)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn actualizar_profesor(
    pool: &PgPool,
    id: i32,
    profesor: Profesor,
) -> Result<(), sqlx::Error> {

    query(
        "UPDATE profesores
        SET nombre = $1,
            correo = $2,
            especialidad = $3
        WHERE id = $4"
    )
    .bind(profesor.nombre)
    .bind(profesor.correo)
    .bind(profesor.especialidad)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn eliminar_profesor(
    pool: &PgPool,
    id: i32,
) -> Result<(), sqlx::Error> {

    query(
        "DELETE FROM profesores WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
