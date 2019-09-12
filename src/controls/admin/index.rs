use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::http::StatusCode;

use super::super::super::models;
use super::super::super::common::define;

#[get("/admin")]
pub fn index(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../../static/views/admin.html")))
}

