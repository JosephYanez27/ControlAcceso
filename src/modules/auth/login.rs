use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::{Utc, Duration};
use serde::Deserialize;
use sqlx::FromRow;
use bcrypt::verify;

use crate::config::jwt::generar_token;
use crate::models::claims::Claims;

/* ========================
   MODELO USUARIO
======================== */
#[derive(FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub pwd: String,
    pub id_perfil: i32,
    pub estado: bool
}

/* ========================
   REQUEST LOGIN
======================== */
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

    // Buscar usuario
    let result = sqlx::query_as::<_, Usuario>(
        "SELECT id, nombre, pwd, id_perfil, estado
         FROM usuario
         WHERE nombre = $1"
    )
    .bind(&form.nombre)
    .fetch_optional(pool.get_ref())
    .await;

    let user = match result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized()
                .body("Usuario no encontrado");
        }
        Err(e) => {
            println!("Error BD: {:?}", e);
            return HttpResponse::InternalServerError()
                .body("Error de base de datos");
        }
    };

    // Verificar estado
    if !user.estado {
        return HttpResponse::Unauthorized()
            .body("Usuario inactivo");
    }
println!("Password enviada: {}", form.pwd);
println!("Password BD: {}", user.pwd);
    // Verificar contraseña
    let password_valida = verify(&form.pwd, &user.pwd)
        .unwrap_or(false);

    if !password_valida {
        return HttpResponse::Unauthorized()
            .body("Contraseña incorrecta");
    }

    // Crear expiración
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

    HttpResponse::Ok().json(token)
}

/* ========================
   REGISTRAR RUTA
======================== */
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}