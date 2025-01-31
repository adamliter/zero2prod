// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/healthz", web::get().to(healthz)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
