use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Profesor {
    pub id: i32,
    pub nombre: String,
    pub correo: String,
    pub especialidad: String,
}
