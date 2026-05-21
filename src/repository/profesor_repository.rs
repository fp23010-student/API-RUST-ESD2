use sqlx::PgPool;
use crate::models::profesor::Profesor;

pub async fn obtener_profesores(pool: &PgPool) -> Result<Vec<Profesor>, sqlx::Error> {
    let profesores = sqlx::query_as!(
        Profesor,
        r#"
        SELECT id, nombre, apellido, correo, especialidad
        FROM profesores
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(profesores)
}
