use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profesor {
    pub id: Option<i32>,
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub especialidad: String,
}
