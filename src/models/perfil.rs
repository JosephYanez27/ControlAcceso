use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Perfil {
    pub id: i32,
    pub strnombre: String,
    pub estado: bool,
}

/* ==========================
   DTO PARA CREAR / EDITAR
========================== */

#[derive(Debug, Deserialize)]
pub struct PerfilRequest {
    pub strnombre: String,
    pub estado: bool,
}