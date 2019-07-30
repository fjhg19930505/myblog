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

/// favicon handler
fn js(req: HttpRequest, path: web::Path<(String, String)>) -> Result<fs::NamedFile> {
    if path.0 != "" {
        if path.1 != "" {
            return Ok(fs::NamedFile::open(format!("static/js/{}/{}", path.0, path.1))?);
        }
    }
    return Ok(fs::NamedFile::open(format!("static/js/{}", path.0))?);
}

fn img(req: HttpRequest, path: web::Path<(String, String)>) -> Result<fs::NamedFile> {
    if path.0 != "" {
        if path.1 != "" {
            return Ok(fs::NamedFile::open(format!("static/img/{}/{}", path.0, path.1))?);
        }
    }
    return Ok(fs::NamedFile::open(format!("static/img/{}", path.0))?);
}

fn css(req: HttpRequest, path: web::Path<(String,)>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(format!("static/css/{}", path.0))?)
}

fn fonts(req: HttpRequest, path: web::Path<(String,)>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(format!("static/fonts/{}", path.0))?)
}

#[get("/")]
fn index(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/views/index.html")))
}

/// 404 handler
fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/views/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

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
            .service(index)
            // with path parameters
            .service(web::resource("/js/{name}/{name2}").route(web::get().to(js)))
            .service(web::resource("/img/{name}/{name2}").route(web::get().to(img)))
            .service(web::resource("/css/{name}").route(web::get().to(css)))
            .service(web::resource("/fonts/{name}").route(web::get().to(fonts)))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Starting http port :8080");
    sys.run()
}