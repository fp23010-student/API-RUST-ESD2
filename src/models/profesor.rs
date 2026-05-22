use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Profesor {
    pub id: i32,
    pub nombre: String,
    pub correo: String,
    pub especialidad: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProfesor {
    pub nombre: String,
    pub correo: String,
    pub especialidad: String,
}
