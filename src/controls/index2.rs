use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub fn go() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}