use actix_web::{App, HttpServer, web};
use actix_files::Files;
use dotenv::dotenv;
use std::env;

mod config;
mod db;
mod middleware;
mod models;
mod modules;
mod services;

use crate::middleware::jwt_middleware::JwtMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    // =========================
    // CONEXIÓN A BASE DE DATOS
    // =========================
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no definida");

    let pool: sqlx::PgPool =
        db::connection::init_pool(&database_url).await;

    // =========================
    // PUERTO (IMPORTANTE EN RENDER)
    // =========================
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Servidor corriendo en puerto {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))

            // =========================
            // ARCHIVOS ESTÁTICOS (HTML, JS, CSS)
            // =========================
          .service(
    Files::new("/static", "./static")
)
.route("/", web::get().to(|| async {
    actix_files::NamedFile::open("./static/login.html")
}))

            // =========================
            // LOGIN (SIN JWT)
            // =========================
            .configure(modules::auth::init)

            // =========================
            // API PROTEGIDA CON JWT
            // =========================
            .service(
                web::scope("/api")
                    .wrap(JwtMiddleware) // 👈 SOLO protege /api/*
                    .configure(modules::seguridad::init)
            )
    })
    .bind(format!("0.0.0.0:{}", port))? // 👈 CLAVE para Render
    .run()
    .await
}