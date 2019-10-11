use crate::interface::controllers::index_controllers;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(index_controllers::index))
            .route(
                "/path_params",
                web::get().to(index_controllers::path_params),
            )
            .route(
                "/body_params",
                web::get().to(index_controllers::body_params),
            )
            .route(
                "/query_params",
                web::get().to(index_controllers::query_params),
            ),
    );
}
