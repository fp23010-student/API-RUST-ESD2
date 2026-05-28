
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration; 

pub fn obtener_url_base_datos() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL no está configurada en el archivo .env")
}

pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let url_base_datos = obtener_url_base_datos();


    PgPoolOptions::new()
        .max_connections(5)                  
        .min_connections(1)                  
        .acquire_timeout(Duration::from_secs(3)) 
        .idle_timeout(Duration::from_secs(15))   
        .max_lifetime(Duration::from_secs(300))  
        .connect(&url_base_datos)
        .await
}