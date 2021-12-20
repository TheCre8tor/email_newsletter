//! lib.rs

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer,};
use serde::{ Deserialize };
use std::io;
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
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
