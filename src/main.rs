// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
