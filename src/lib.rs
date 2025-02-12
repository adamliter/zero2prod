// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(|| App::new().route("/healthz", web::get().to(healthz)))
            .listen(listener)?
            .run();

    Ok(server)
}
