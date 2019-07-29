use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod controls;

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controls::index::go))
            .route("/again", web::get().to(controls::index2::go))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
