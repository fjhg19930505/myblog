use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::http::StatusCode;

use super::super::super::models;

#[derive(Debug)]
enum VerifyResult {
    VerifyResult_,
    Variant2,
}

#[get("/admin")]
pub fn index(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../../static/views/admin.html")))
}

pub fn verify(phone: i32, code i32) -> Result<VerifyResult> {
    models::admin_model::verify_code()
}