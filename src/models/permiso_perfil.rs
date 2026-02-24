use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PermisoPerfil {
    pub id: i32,
    pub id_perfil: i32,
    pub id_modulo: i32,
    pub crear: bool,
    pub editar: bool,
    pub eliminar: bool,
}
#[derive(Deserialize)]
pub struct PermisoPerfilRequest {
    pub id_perfil: i32,
    pub id_modulo: i32,
    pub crear: bool,
    pub editar: bool,
    pub eliminar: bool,
}