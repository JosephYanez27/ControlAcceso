pub mod usuario;
pub mod perfil;
pub mod permisos;
pub mod menu;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    usuario::init(cfg);
    perfil::init(cfg);
    permisos::init(cfg);
    menu::init(cfg);
}