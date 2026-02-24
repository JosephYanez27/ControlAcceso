use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::perfil::{Perfil, PerfilRequest};


/* ========================
   REGISTRAR RUTAS
======================== */
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/seguridad/perfil")
            .service(listar_perfiles)
            .service(crear_perfil)
            .service(actualizar_perfil)
    );
}

/* ========================
   LISTAR
======================== */
#[get("/")]
async fn listar_perfiles(pool: web::Data<PgPool>) -> impl Responder {

    let perfiles = sqlx::query_as::<_, Perfil>(
        "SELECT id, strnombre, estado FROM perfil"
    )
    .fetch_all(pool.get_ref())
    .await;

    match perfiles {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/* ========================
   CREAR
======================== */
#[post("/")]
async fn crear_perfil(
    pool: web::Data<PgPool>,
    form: web::Json<PerfilRequest>
) -> impl Responder {

    let result = sqlx::query(
        "INSERT INTO perfil (strnombre, estado)
         VALUES ($1, $2)"
    )
    .bind(&form.strnombre)
    .bind(form.estado)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Perfil creado"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/* ========================
   EDITAR
======================== */
#[put("/{id}")]
async fn actualizar_perfil(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    form: web::Json<PerfilRequest>
) -> impl Responder {

    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE perfil
         SET strnombre = $1,
             estado = $2
         WHERE id = $3"
    )
    .bind(&form.strnombre)
    .bind(form.estado)
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Perfil actualizado"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}