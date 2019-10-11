use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IndexResponse {
    message: String,
}

pub fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(IndexResponse {
            message: "this api is working correctly! ver 1.0.0".to_string(),
        })
}
