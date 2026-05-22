use axum::{
    routing::get,
    Router,
};
use controller::carrera_controller;
use controller::profesor_controller;

mod config;
mod models;
mod repository;
mod service;
mod controller;

#[tokio::main]
async fn main() {
    let pool = config::config::crear_pool()
        .await
        .expect("Error al crear el pool de conexiones");

    let app = Router::new()
        .route("/carreras", get(carrera_controller::get_all).post(carrera_controller::create))
        .route("/carreras/{id}", get(carrera_controller::get_by_id)
                                .put(carrera_controller::update)
                                .delete(carrera_controller::delete))
        .route("/profesores", get(profesor_controller::get_all).post(profesor_controller::create))
        .route("/profesores/:id", 
            get(profesor_controller::get_by_id)
            .put(profesor_controller::update)
            .delete(profesor_controller::delete)
               
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Servidor corriendo en http://127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();
}
