//! main.rs

use email_newsletter::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error is we failed to bind the address
    // Otherwise call .await on our Server

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    run(listener)?.await
}
