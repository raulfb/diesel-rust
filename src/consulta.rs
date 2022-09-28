extern crate rust_diesel;
extern crate diesel;

use self::rust_diesel::*;
use self::models::*;
use self::diesel::prelude::*;
fn main(){
    get_usuarios(); 
}

fn get_usuarios(){
    let connection = &mut establish_connection();
    use self::schema::usuarios::dsl::*;
    let results = usuarios       
        .limit(5)
        .load::<Usuarios>(connection)
        .expect("Error loading posts");
    println!("-----------------------------------------");
    println!("Se encontraron {} usuarios  ", results.len());
    println!("----------------------------------------\n");
    for usuario in results{
        let estado_usuario;
        if usuario.activo == true{
            estado_usuario="Activo".to_string();
        }else{
            estado_usuario="Inactivo".to_string();
        }
        println!("Nombre usuario: {}, apellidos: {} , estado {} ",usuario.nombre,usuario.apellidos,estado_usuario);
    }
}

// fn get_usuario(){

// }