pub mod login;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    login::init(cfg);
}