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

