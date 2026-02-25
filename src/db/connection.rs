use sqlx::{PgPool, postgres::PgPoolOptions};


use std::time::Duration;

pub async fn init_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await
        .expect("No se pudo conectar a la base de datos")
}