use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Inscripcion {
    pub id_inscripcion: i32,
    pub id_estudiante: Option<i32>,
    pub id_materia: Option<i32>,
    pub ciclo: String,
    pub nota_final: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInscripcionDto {
    pub id_estudiante: Option<i32>,
    pub id_materia: Option<i32>,
    pub ciclo: String,
    pub nota_final: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInscripcionDto {
    pub id_estudiante: Option<i32>,
    pub id_materia: Option<i32>,
    pub ciclo: Option<String>,
    pub nota_final: Option<f64>,
}