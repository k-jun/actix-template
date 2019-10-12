use actix_web::{
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse, Responder,
};

use crate::models::index_models;

pub fn index(_req: HttpRequest) -> impl Responder {
  "Hello World!"
}

pub fn path((path, req): (Path<index_models::PathPath>, HttpRequest)) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("application/json")
    .json(index_models::PathResponse {
      string: path.string.to_string(),
      integer: path.integer,
      float: path.float,
    })
}

pub fn query((query, req): (Query<index_models::QueryQuery>, HttpRequest)) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("application/json")
    .json(index_models::QueryResponse {
      string: query.string.to_string(),
      integer: query.integer,
      float: query.float,
    })
}

pub fn body((body, req): (Json<index_models::BodyJson>, HttpRequest)) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("application/json")
    .json(index_models::BodyResponse {
      string: body.string.to_string(),
      integer: body.integer,
      float: body.float,
    })
}