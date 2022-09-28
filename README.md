## Diesel-rust
Proyecto para probar Diesel.
## Instalar diesel.

Para instalar diesel hay que añadir la siguiente dependencia en el archivo Cargo.toml:
```rust
diesel = { version = "2.0.0", features = ["mysql"] }
```

## Instalar diesel_cli

Al instalar diesel_cli da un error. Se soluciona instalando solo la feature para mysql.
```rust
cargo install diesel_cli --no-default-features --features mysql
```
## Configurar .env
Generamos un archivo .env con el siguiente contenido:
```rust
DATABASE_URL=mysql://usuario:contraseña@localhost/base_de_datos
```
## Setup
Ejecutamos el siguiente comando:

```rust
diesel setup
```

Este comando nos crea la bd si no existe. Aparte crea el directorio migrations y el archivo diesel.toml

## Migraciones

### Crear una migración:

Comando:

```rust
diesel migration generate crear_usuario
```

Contenido archivo up.sql

```rust
CREATE TABLE usuarios (
  id INT NOT NULL AUTO_INCREMENT,
  nombre VARCHAR(100) NOT NULL,
  apellidos VARCHAR(100) NOT NULL,
  activo BOOLEAN NOT NULL DEFAULT FALSE,
  PRIMARY KEY (id)
)
```
Contenido archivo down.sql

```rust
DROP TABLE usuarios
```

### Ejecutar una migración:

```rust
diesel migration run
```

### Eliminar una migración:

```rust
diesel migration redo
```
## Código

### Conectarse a la bd
```rust
pub fn establecer_conexion_bc() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```

### Get Usuarios
```rust

fn get_usuarios(){
    let connection = &mut establecer_conexion_bc();
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
```
### Get Usuario
```rust
fn get_usuario(nombre_usuario:String){
    let connection = &mut establecer_conexion_bc();
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
```

