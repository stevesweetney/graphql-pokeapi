use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::future::Future;
use juniper;
use juniper::http::GraphQLRequest;
use std::io;
use std::sync::Arc;

mod api;
mod schema;
mod types;

use schema::{create_schema, Schema};

fn index() -> impl Responder {
    "Hello from actix"
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        serde_json::to_string(&res)
    })
    .map_err(Error::from)
    .and_then(|response| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(response))
    })
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
