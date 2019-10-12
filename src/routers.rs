use crate::controllers::index_controllers;
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
  app
    .service(web::resource("/").to(index_controllers::index))
    .service(
      web::scope("/params")
        .route(
          "path/{string}/{integer}/{float}",
          web::get().to(index_controllers::path),
        )
        .route("query", web::get().to(index_controllers::query))
        .route("body", web::post().to(index_controllers::body)),
    )
    .service(web::scope("/todo").route("path", web::post().to(index_controllers::index)));
}
