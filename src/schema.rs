// @generated automatically by Diesel CLI.

diesel::table! {
    usuarios (id) {
        id -> Integer,
        nombre -> Varchar,
        apellidos -> Varchar,
        activo -> Bool,
    }
}
