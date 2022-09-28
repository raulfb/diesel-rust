use crate::schema::usuarios;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Usuarios {
    pub id: i32,
    pub nombre: String,
    pub apellidos: String,
    pub activo: bool,
}

#[derive(Insertable,AsChangeset)]
#[diesel(table_name = usuarios)]
pub struct NuevoUsuario<'a> {
    pub nombre: &'a String,
    pub apellidos: &'a String,
    pub activo: &'a bool,
}

