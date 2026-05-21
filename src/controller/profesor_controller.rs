use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::service::profesor_service;

pub async fn get_profesores(
    pool: web::Data<PgPool>,
) -> impl Responder {

    match profesor_service::listar_profesores(pool.get_ref()).await {
        Ok(profesores) => HttpResponse::Ok().json(profesores),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
