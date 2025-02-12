// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    run(listener).expect("Failed to bind address.").await
}
