use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

#[path = "./routes/mod.rs"]
mod routes;

use routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, Error> {
    // Wrap the connection in a smart pointer -->
    let db_pool = web::Data::new(db_pool);

    // Capture `connection` from the surrounding environment -->
    let server = HttpServer::new(move || {
        /* App is where all application logic lives:
        routing, middlewares, request handlers, etc */
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state -->
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
