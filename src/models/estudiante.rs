use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Estudiante {
    pub id_estudiante: i32,
    pub nombre: String,
    pub apellido: String,
    pub carnet: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_carrera: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEstudianteDto {
    pub nombre: String,
    pub apellido: String,
    pub carnet: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_carrera: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEstudianteDto {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub carnet: Option<String>,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_carrera: Option<i32>,
}