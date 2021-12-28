use actix_web::{web, App, HttpServer, dev::{Server}};
use std::io::Error;
use std::net::TcpListener;

#[path = "./routes/mod.rs"]
mod routes;

use routes::{subscribe, health_check};


pub fn run(listener: TcpListener) -> Result<Server, Error> {
    // HttpServer handles all transport level concerns -->
    let server = HttpServer::new(|| {
        /* App is where all application logic lives:
        routing, middlewares, request handlers, etc */
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}