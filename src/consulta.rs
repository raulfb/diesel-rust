extern crate rust_diesel;
extern crate diesel;
use crate::schema::usuarios;

use self::rust_diesel::*;
use self::models::*;
use self::diesel::prelude::*;
use self::models::{NuevoUsuario};
fn main(){

    // get_usuarios(); 
    // let id_usuario=7;
    let nombre_usuario=String::from("Zacariass");
    get_usuario(nombre_usuario);
    
    // let nombre= String::from("Zacarias");
    // let apellidos = String::from("Sanchez Martinez");
    // let activo= true;

    // crear_usuario(&nombre,&apellidos,&activo);

    // let id_usuario=6;
    // eliminar_usuario(id_usuario);

    // let id_usuario=7;
    // actualizar_usuario(id_usuario);
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

fn get_usuario(nombre_usuario:String){
    let connection = &mut establish_connection();
    use self::schema::usuarios::dsl::*;
    let results = usuarios
        .filter(nombre.eq(&nombre_usuario))
        .filter(id.eq(21))       
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
            println!("No existe el usuario {}",nombre_usuario);
        }
}


fn crear_usuario(nombre:&String,apellidos:&String,activo:&bool){
    let connection = &mut establish_connection();
    let nuevo_usuario = NuevoUsuario {nombre, apellidos,activo };

    let usuario_creado=diesel::insert_into(usuarios::table)
        .values(nuevo_usuario)
        .execute(connection)
        .expect("Error creando el usuario");
    if usuario_creado==1{
        println!("Usuario creado.");
    }else{
        println!("Se ha producido un error al crear el usuario");
    }
}

fn eliminar_usuario(id_usuario:i32){
    let connection = &mut  establish_connection();
    use self::schema::usuarios::dsl::*;
    let usuario_eliminado = diesel::delete(usuarios.filter(id.eq(id_usuario)))
    .execute(connection)
    .expect("Error eliminando el usuario");
    if usuario_eliminado==1{
        println!("Usuario con id {} eliminado correctamente!",id_usuario);
    }else{
        println!("Se ha producido un error al eliminar el usuario con id {}",id_usuario)
    }  
}

fn actualizar_usuario(id_usuario:i32){
    use self::schema::usuarios::dsl::*;
    let connection = &mut establish_connection();
    let usuario_actualizado=diesel::update(usuarios.filter(id.eq(id_usuario)))
    .set((nombre.eq("James"), apellidos.eq("Not Bond")))
    .execute(connection).unwrap();
    if usuario_actualizado==1{
        println!("Usuario con id {} actualizado correctamente!",id_usuario)
    }else{
        println!("Se ha producido un error al actualizar el usuario con id {}",id_usuario)
    }
}