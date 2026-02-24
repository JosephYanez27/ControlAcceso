use actix_web::{get, post, put, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::usuario::{Usuario, UsuarioRequest};
use crate::services::permiso_service::tiene_acceso;
use crate::models::claims::Claims;
use crate::models::permiso_perfil::PermisoPerfil;

/* ========================
   REGISTRAR RUTAS
======================== */

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/seguridad/usuario")
            .service(listar_usuarios)
            .service(crear_usuario)
            .service(actualizar_usuario)
    );
}

/* ========================
   LISTAR
======================== */

#[get("/")]
async fn listar_usuarios(
    pool: web::Data<PgPool>,
    req: HttpRequest
) -> impl Responder {

    let extensions = req.extensions();

    let claims = match extensions.get::<Claims>() {
        Some(c) => c.clone(),
        None => return HttpResponse::Unauthorized().finish(),
    };

    let permiso = sqlx::query_as::<_, PermisoPerfil>(
        "SELECT id, id_perfil, id_modulo, crear, editar, eliminar
         FROM permiso_perfil
         WHERE id_perfil = $1 AND id_modulo = 1"
    )
    .bind(claims.perfil_id)
    .fetch_optional(pool.get_ref())
    .await;

    let permiso = match permiso {
        Ok(Some(p)) => p,
        _ => return HttpResponse::Forbidden().finish()
    };

    if !tiene_acceso(&permiso) {
        return HttpResponse::Forbidden().finish();
    }

    let result = sqlx::query_as::<_, Usuario>(
        "SELECT id, nombre, id_perfil, estado FROM usuario"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(usuarios) => HttpResponse::Ok().json(usuarios),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

/* ========================
   CREAR
======================== */

#[post("/")]
async fn crear_usuario(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    form: web::Json<UsuarioRequest>
) -> impl Responder {

    let extensions = req.extensions();

    let claims = match extensions.get::<Claims>() {
        Some(c) => c.clone(),
        None => return HttpResponse::Unauthorized().finish(),
    };

    let permiso = sqlx::query_as::<_, PermisoPerfil>(
        "SELECT id, id_perfil, id_modulo, crear, editar, eliminar
         FROM permiso_perfil
         WHERE id_perfil = $1 AND id_modulo = 1"
    )
    .bind(claims.perfil_id)
    .fetch_optional(pool.get_ref())
    .await;

    let permiso = match permiso {
        Ok(Some(p)) => p,
        _ => return HttpResponse::Forbidden().finish()
    };

    if !permiso.crear {
        return HttpResponse::Forbidden().finish();
    }

    let hashed = match hash(&form.pwd, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let result = sqlx::query(
        "INSERT INTO usuario (nombre, pwd, id_perfil, estado)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(&form.nombre)
    .bind(&hashed)
    .bind(form.id_perfil)
    .bind(form.estado)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Usuario creado"),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

/* ========================
   EDITAR
======================== */

#[put("/{id}")]
async fn actualizar_usuario(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<i32>,
    form: web::Json<UsuarioRequest>
) -> impl Responder {

    let extensions = req.extensions();

    let claims = match extensions.get::<Claims>() {
        Some(c) => c.clone(),
        None => return HttpResponse::Unauthorized().finish(),
    };

    let permiso = sqlx::query_as::<_, PermisoPerfil>(
        "SELECT id, id_perfil, id_modulo, crear, editar, eliminar
         FROM permiso_perfil
         WHERE id_perfil = $1 AND id_modulo = 1"
    )
    .bind(claims.perfil_id)
    .fetch_optional(pool.get_ref())
    .await;

    let permiso = match permiso {
        Ok(Some(p)) => p,
        _ => return HttpResponse::Forbidden().finish()
    };

    if !permiso.editar {
        return HttpResponse::Forbidden().finish();
    }

    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE usuario
         SET nombre = $1,
             id_perfil = $2,
             estado = $3
         WHERE id = $4"
    )
    .bind(&form.nombre)
    .bind(form.id_perfil)
    .bind(form.estado)
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Usuario actualizado"),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}