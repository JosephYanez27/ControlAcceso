use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use actix_files::Files;
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

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no definida");

let pool: sqlx::PgPool =
    db::connection::init_pool(&database_url).await;

    HttpServer::new(move || {
        App::new()
            
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(JwtMiddleware)
            .service(Files::new("/static", "./static").show_files_listing())
            .configure(modules::auth::init)
            .configure(modules::seguridad::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}