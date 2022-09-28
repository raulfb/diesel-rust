extern crate rust_diesel;
extern crate diesel;

use self::rust_diesel::*;
use self::models::*;
use self::diesel::prelude::*;
fn main(){
    // get_usuarios(); 
    let id_usuario=10;
    get_usuario(id_usuario);
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
        .limit(5)
        .load::<Usuarios>(connection)
        .expect("Error obteniendo el usuario");
        // println!("{:?}",results);
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