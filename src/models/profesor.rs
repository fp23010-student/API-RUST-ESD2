use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Profesor {
    pub id_profesor: i32,
    pub nombre: String,
    pub especialidad: String,
    pub correo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProfesor {
    pub nombre: String,
    pub especialidad: String,
    pub correo: String,
}
