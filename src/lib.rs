pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NuevoUsuario, Usuarios};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut MysqlConnection, nombre: &String, apellidos: &String,activo:&bool) -> Usuarios {
    use crate::schema::usuarios;

    let nuevo_usuario = NuevoUsuario { nombre, apellidos,activo };

    diesel::insert_into(usuarios::table)
        .values(&nuevo_usuario)
        .execute(conn)
        .expect("Error saving new post");

    usuarios::table.order(usuarios::id.desc()).first(conn).unwrap()
}