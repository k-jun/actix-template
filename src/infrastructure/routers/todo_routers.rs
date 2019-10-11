use crate::interface::controllers::todo_controllers;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/todo")
            .route(web::get().to(todo_controllers::index))
            .route(
                web::post().to(|| HttpResponse::Ok().body("this is post resource index routing")),
            )
            .route(
                web::delete()
                    .to(|| HttpResponse::Ok().body("this is delete resource index routing")),
            )
            .route(
                web::put().to(|| HttpResponse::Ok().body("this is update resource index routing")),
            ),
    );
}
