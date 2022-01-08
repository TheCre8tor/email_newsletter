//! main.rs

#[path = "./lib.rs"]
mod lib;

use email_newsletter::configuration::get_configuration;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

use env_logger::Env;

#[tokio::main]
async fn main() -> io::Result<()> {
    // `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Bubble up the io::Error if we failed to bind the address
    let listener = TcpListener::bind(address)?;
    // Otherwise call .await on our Server
    lib::startup::run(listener, connection_pool)?.await
}
