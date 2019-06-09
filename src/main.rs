use actix_web::{web, App, HttpServer, Responder};
use std::io;

mod api;
mod schema;
mod types;

fn index() -> impl Responder {
    "Hello from actix"
}

fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind("127.0.0.1:8080")?
        .run()
}
