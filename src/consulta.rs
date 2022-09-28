extern crate rust_diesel;
extern crate diesel;

use self::rust_diesel::*;
use self::models::*;
use self::diesel::prelude::*;
fn main(){

    // get_usuarios(); 
    // let id_usuario=1;
    // get_usuario(id_usuario);
    
    let nombre= String::from("Zacarias");
    let apellidos = String::from("Sanchez Martinez");
    let activo= true;

    crear_usuario(nombre,apellidos,activo);
}


fn get_usuarios(){
    let connection = &mut establish_connection();
    use self::schema::usuarios::dsl::*;
    let results = usuarios       
        .limit(5)
        .load::<Usuarios>(connection)
        .expect("Error obteniendo los usuarios");

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

fn get_usuario(id_usuario:i32){
    let connection = &mut establish_connection();
    use self::schema::usuarios::dsl::*;
    let results = usuarios
        .filter(id.eq(id_usuario))       
        .limit(1)
        .load::<Usuarios>(connection)
        .expect("Error obteniendo el usuario");

        if results.len()>0{
            let estado_usuario;
            if results[0].activo == true{
                estado_usuario="Activo".to_string();
            }else{
                estado_usuario="Inactivo".to_string();
            }
           println!("Nombre usuario: {}, apellidos: {} , estado {} ",results[0].nombre,results[0].apellidos,estado_usuario);
        }else{
            println!("No existe el usuario con id {}",id_usuario);
        }
}


fn crear_usuario(nombre:String,apellidos:String,activo:bool){
    let connection = &mut establish_connection();
    let usuario = create_user(connection, &nombre, &apellidos,&activo);
    println!("Usuario guardado.");
    println!("Datos del nuevo usuario: id: {} nombre: {} apellidos {} activo {}",usuario.id,usuario.nombre,usuario.apellidos,usuario.activo);
}
