/***
 * configuración de la aplicación en base de datos y otros parámetros
 * se configuro la conexión a la base de datos utilizando sqlx y se establecieron los parámetros necesarios para la conexión de postgreSQL.
 * Además, se definieron las rutas para el servidor utilizando axum y se configuró el puerto de escucha para el servidor.
 */
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration; // <-- 1. Importamos esto para manejar los tiempos de espera

pub fn obtener_url_base_datos() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL no está configurada en el archivo .env")
}

pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let url_base_datos = obtener_url_base_datos();

    // 2. Agregamos las opciones de estabilidad para el Pooler de Supabase
    PgPoolOptions::new()
        .max_connections(5)                  // Máximo de conexiones simultáneas
        .min_connections(1)                  // Mantiene al menos una conexión lista
        .acquire_timeout(Duration::from_secs(3)) // Si una conexión se traba, rompe el "Processing..." a los 3 segundos en vez de colgarse para siempre
        .idle_timeout(Duration::from_secs(15))   // Elimina del pool las conexiones inactivas antes de que Supabase las mate silenciosamente
        .max_lifetime(Duration::from_secs(300))  // Recicla conexiones viejas cada 5 minutos para evitar corrupción de sockets
        .connect(&url_base_datos)
        .await
}