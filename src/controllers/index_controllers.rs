use crate::models::index_models;
use actix_web::{
  error::Error,
  web::{Json, Path, Query},
  HttpRequest, HttpResponse, Responder,
};
#[macro_use]
use serde_json::json;

pub fn index(_req: HttpRequest) -> impl Responder {
  "Hello World!"
}

pub fn path(path: Path<index_models::PathPath>) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .json(json!({
        "f_str": path.f_str.to_string(),
        "f_int": path.f_int,
        "f_flt": path.f_flt,
      })),
  )
}

pub fn query(query: Query<index_models::QueryQuery>) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .json(json!({
        "f_str": query.f_str.to_string(),
        "f_int": query.f_int,
        "f_flt": query.f_flt,
      })),
  )
}

pub fn body(body: Json<index_models::BodyJson>) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .json(json!({
        "f_str": body.f_str.to_string(),
        "f_int": body.f_int,
        "f_flt": body.f_flt,
      })),
  )
}
