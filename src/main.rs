//! main.rs
use std::io;

use email_newsletter::configuration::get_configuration;
use email_newsletter::startup;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    /* ------- Telemetry & Logs Config ------- */
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), io::stdout);
    init_subscriber(subscriber);

    /* ------- Main App ------- */

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.with_db());

    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    // Bubble up the io::Error if we failed to bind the address
    let listener = TcpListener::bind(address)?;
    // Otherwise call .await on our Server
    startup::run(listener, connection_pool)?.await
}
