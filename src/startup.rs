use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);

    // Capture `connection` from the surrounding environment -->
    let server = HttpServer::new(move || {
        /* App is where all application logic lives:
        routing, middlewares, request handlers, etc */
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state -->
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
