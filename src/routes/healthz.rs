// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
use actix_web::{HttpResponse, Responder};

pub async fn healthz() -> impl Responder {
    HttpResponse::Ok().finish()
}
