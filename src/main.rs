use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::io;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // HttpServer handles all transport level concerns -->
    HttpServer::new(|| {
        /* App is where all application logic lives:
           routing, middlewares, request handlers, etc */ 
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
