use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::{Utc, Duration};
use serde::Deserialize;
use sqlx::FromRow;
use bcrypt::verify;
use crate::config::jwt::generar_token;

use crate::models::claims::Claims;

/* ========================
   MODELO USUARIO PARA LOGIN
======================== */
#[derive(FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub pwd: String,
    pub id_perfil: i32,
    pub estado: bool
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub nombre: String,
    pub pwd: String
}

/* ========================
   LOGIN
======================== */
#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,
    form: web::Json<LoginRequest>
) -> impl Responder {

    let user = sqlx::query_as::<_, Usuario>(
        "SELECT id, nombre, pwd, id_perfil, estado
         FROM usuario
         WHERE nombre = $1"
    )
    .bind(&form.nombre)
    .fetch_one(pool.get_ref())
    .await;

    if let Ok(user) = user {
        if !user.estado {
        return HttpResponse::Unauthorized().body("Usuario inactivo");
    }
        if verify(&form.pwd, &user.pwd).unwrap_or(false) {

            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(2))
                .unwrap()
                .timestamp() as usize;

            let claims = Claims {
                sub: user.nombre,
                perfil_id: user.id_perfil,
                exp: expiration,
            };

           let token = generar_token(claims);

            return HttpResponse::Ok().json(token);
        }
    }

    HttpResponse::Unauthorized().finish()
}

/* ========================
   REGISTRAR RUTA
======================== */
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}