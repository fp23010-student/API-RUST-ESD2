use sqlx::PgPool;

use crate::models::profesor::Profesor;
use crate::repository::profesor_repository;

pub async fn listar_profesores(
    pool: &PgPool,
) -> Result<Vec<Profesor>, sqlx::Error> {

    profesor_repository::obtener_profesores(pool).await
}
