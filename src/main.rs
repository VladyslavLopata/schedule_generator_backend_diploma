use crate::routes::login::login;
use crate::routes::schedule::hello;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod models;
mod routes;
use models::teacher;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(login)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
