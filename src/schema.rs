// @generated automatically by Diesel CLI.

diesel::table! {
    mapas (id) {
        id -> Integer,
        nombre -> Varchar,
        descripcion -> Varchar,
    }
}

diesel::table! {
    usuarios (id) {
        id -> Integer,
        nombre -> Varchar,
        apellidos -> Varchar,
        activo -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    mapas,
    usuarios,
);
