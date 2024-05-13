// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
pub mod configuration;
pub mod routes;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthz", web::get().to(routes::healthz))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
