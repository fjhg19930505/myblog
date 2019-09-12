#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{guard, error, middleware, web, App, Error, HttpResponse, HttpServer};
use actix_session::{CookieSession};
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::{io, env};

mod controls;
mod models;
mod common;

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("myblog");


    // controls路由
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
            .service(controls::admin::index::index)

            // 以下是数据库相关的路由
            .service(web::resource("/verify/{phone}/{code}").route(web::post().to_async(models::admin_model::verify_code)))
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

    // models路由
    /*let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // This can be called with:
            // curl -S --header "Content-Type: application/json" --request POST --data '{"name":"xyz"}'  http://127.0.0.1:8080/add
            // Use of the extractors makes some post conditions simpler such
            // as size limit protections and built in json validation.
            .service(
                web::resource("/add2")
                    .data(
                        web::JsonConfig::default()
                            .limit(4096) // <- limit size of the payload
                            .error_handler(|err, _| {
                                // <- create custom error response
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                )
                                .into()
                            }),
                    )
                    .route(web::post().to_async(add2)),
            )
            //  Manual parsing would allow custom error construction, use of
            //  other parsers *beside* json (for example CBOR, protobuf, xml), and allows
            //  an application to standardise on a single parser implementation.
            //.service(web::resource("/verify/{phone}/{code}").route(web::post().to_async(controls::admin::index::verify)))
            //.service(web::resource("/add/{phone}/{name}").route(web::get().to_async(models::admin_model::add)))
    })
    //.bind("127.0.0.1:8088")?
    //.start();*/
    
    sys.run()
}
