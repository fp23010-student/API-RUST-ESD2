use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Carrera {
    pub id_carrera: i32,
    pub nombre_carrera: String,
    pub facultad: String,
}

#[derive(Serialize, Deserialize)]
pub struct CarreraInput {
    pub nombre_carrera: String,
    pub facultad: String,
}