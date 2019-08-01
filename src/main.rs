#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use bytes::Bytes;
use futures::unsync::mpsc;
use futures::{future::ok, Future, Stream};

mod controls;

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("myblog");

    HttpServer::new(|| {
        App::new()
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register simple route, handle all methods
            .service(controls::index::index)
            // with path parameters
            .service(web::resource("/js/{name1}").route(web::get().to(controls::common::js)))
            .service(web::resource("/img/{name1}").route(web::get().to(controls::common::img)))
            .service(web::resource("/css/{name}").route(web::get().to(controls::common::css)))
            .service(web::resource("/fonts/{name}").route(web::get().to(controls::common::fonts)))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(controls::common::p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .bind("192.168.1.39:8080")?
    .start();

    println!("Starting http port :8080");
    sys.run()
}