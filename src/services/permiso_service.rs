use crate::models::permiso_perfil::PermisoPerfil;
pub fn tiene_acceso(p: &PermisoPerfil) -> bool {
    p.crear || p.editar || p.eliminar
}