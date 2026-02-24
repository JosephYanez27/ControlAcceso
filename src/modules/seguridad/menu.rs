use actix_web::{get, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use sqlx::PgPool;
use crate::models::claims::Claims;
use crate::models::permiso_perfil::PermisoPerfil;

/* ========================
   OBTENER MENU SEGÚN PERMISOS
======================== */

#[get("/menu")]
async fn obtener_menu(
    pool: web::Data<PgPool>,
    req: HttpRequest
) -> impl Responder {

    let extensions = req.extensions();

    let claims = match extensions.get::<Claims>() {
        Some(c) => c.clone(),
        None => return HttpResponse::Unauthorized().finish()
    };

    let permisos = sqlx::query_as::<_, PermisoPerfil>(
        "SELECT id, id_perfil, id_modulo, crear, editar, eliminar
         FROM permiso_perfil
         WHERE id_perfil = $1"
    )
    .bind(claims.perfil_id)
    .fetch_all(pool.get_ref())
    .await;

    let permisos = match permisos {
        Ok(p) => p,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    // Verificar si tiene algún permiso en seguridad
    let tiene_seguridad = permisos.iter().any(|p|
        p.crear || p.editar || p.eliminar
    );

    HttpResponse::Ok().json(serde_json::json!({
        "seguridad": tiene_seguridad,
        "permisos": permisos
    }))
}

/* ========================
   REGISTRAR RUTA
======================== */

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/seguridad")
            .service(obtener_menu)
    );
}