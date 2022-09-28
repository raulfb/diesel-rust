use diesel::prelude::*;

#[derive(Queryable)]
pub struct Usuarios {
    pub id: i32,
    pub nombre: String,
    pub apellidos: String,
    pub activo: bool,
}