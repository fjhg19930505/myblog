use actix_web::{HttpRequest, HttpResponse};
use futures::stream::once;
use bytes::Bytes;
use actix_http::body::Body;

pub fn go(req: &HttpRequest) -> HttpResponse {
    let body = once(Ok(Bytes::from_static(b"views/index")));

    HttpResponse::Ok()
        .content_type("application/json")
        .body(Body::Streaming(Box::new(body)))
}