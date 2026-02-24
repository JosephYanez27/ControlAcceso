use serde::{Serialize, Deserialize};


#[derive(Serialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub id_perfil: i32,
    pub estado: bool
}

#[derive(Deserialize)]
pub struct UsuarioRequest {
    pub nombre: String,
    pub pwd: String,
    pub id_perfil: i32,
    pub estado: bool
}