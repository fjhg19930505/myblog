#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_session::{CookieSession};
use actix_web::{
    guard, middleware, web, App, HttpResponse, HttpServer,
};

mod controls;
mod admin;

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
            .service(web::resource("/js/{name1}").route(web::get().to(controls::common::js1)))
            .service(web::resource("/js/{name1}/{name2}").route(web::get().to(controls::common::js2)))
            .service(web::resource("/img/{name1}").route(web::get().to(controls::common::img1)))
            .service(web::resource("/img/{name1}/{name2}").route(web::get().to(controls::common::img2)))
            .service(web::resource("/css/{name}").route(web::get().to(controls::common::css)))
            .service(web::resource("/fonts/{name}").route(web::get().to(controls::common::fonts)))

            // 以下是管理员的路由
            .service(admin::index::index)
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
    .bind("0.0.0.0:8080")?
    .start();

    println!("Starting http port :8080");
    sys.run()
}