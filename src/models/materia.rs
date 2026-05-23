use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Materia {
    pub id_materia: i32,
    pub nombre_materia: String,
    pub uv: i32,
    pub id_profesor: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MateriaInput {
    pub nombre_materia: String,
    pub uv: i32,
    pub id_profesor: i32,
}