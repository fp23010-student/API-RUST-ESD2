use sqlx::{PgPool, query_as};

use crate::models::profesor::Profesor;

pub async fn obtener_profesores(
    pool: &PgPool,
) -> Result<Vec<Profesor>, sqlx::Error> {

    let profesores = query_as::<_, Profesor>(
        "SELECT id, nombre, apellido, correo, especialidad FROM profesores"
    )
    .fetch_all(pool)
    .await?;

    Ok(profesores)
}
