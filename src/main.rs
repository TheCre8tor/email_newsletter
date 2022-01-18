//! main.rs
use std::io;

use email_newsletter::configuration::get_configuration;
use email_newsletter::startup;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPool;
use std::net::TcpListener;

use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> io::Result<()> {
    /* ------- Telemetry & Logs Config ------- */
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), io::stdout);
    init_subscriber(subscriber);

    /* ------- Main App ------- */

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Bubble up the io::Error if we failed to bind the address
    let listener = TcpListener::bind(address)?;
    // Otherwise call .await on our Server
    startup::run(listener, connection_pool)?.await
}
