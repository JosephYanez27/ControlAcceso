use actix_web::{get, web,post, HttpResponse, Responder, HttpRequest, HttpMessage};
use sqlx::PgPool;
use crate::models::claims::Claims;
use crate::models::permiso_perfil::PermisoPerfil;
use crate::models::permiso_perfil::PermisoPerfilRequest;

/* ========================
   MODELO
======================== */

/* ========================
   OBTENER PERMISOS POR PERFIL
======================== */
#[get("/{perfil_id}")]
async fn obtener_permisos(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<i32>
) -> impl Responder {

    // 👇 SOLUCIÓN AQUÍ
    let extensions = req.extensions();

    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().finish()
    };

    let perfil_id = path.into_inner();

    // Opcional: validar que el usuario solo consulte su propio perfil
    if claims.perfil_id != perfil_id {
        return HttpResponse::Forbidden().finish();
    }

let permisos = sqlx::query_as::<_, PermisoPerfil>(
    "SELECT id, id_modulo, id_perfil, crear, editar, eliminar
     FROM permiso_perfil
     WHERE id_perfil = $1"
)
.bind(perfil_id)
.fetch_all(pool.get_ref())
.await;

match permisos {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(e) => {
        println!("Error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    }
}
}

#[post("/")]
async fn asignar_permiso(
    pool: web::Data<PgPool>,
    form: web::Json<PermisoPerfilRequest>
) -> impl Responder {

    let result = sqlx::query(
        "INSERT INTO permiso_perfil
        (id_perfil, id_modulo, crear, editar, eliminar)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id_perfil, id_modulo)
        DO UPDATE SET
            crear = EXCLUDED.crear,
            editar = EXCLUDED.editar,
            eliminar = EXCLUDED.eliminar"
    )
    .bind(form.id_perfil)
    .bind(form.id_modulo)
    .bind(form.crear)
    .bind(form.editar)
    .bind(form.eliminar)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Permiso guardado"),
        Err(e) => {
            println!("Error guardando permiso: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/* ========================
   REGISTRAR RUTA
======================== */
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/seguridad/permisos")
            .service(obtener_permisos)
            .service(asignar_permiso)
    );
}