use crate::controllers::{index_controllers, todo_controllers};
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
  app
    .service(web::resource("/").to(index_controllers::index))
    .service(
      web::scope("/params")
        .route(
          "path/{f_str}/{f_int}/{f_flt}",
          web::get().to(index_controllers::path),
        )
        .route("query", web::get().to(index_controllers::query))
        .route("body", web::post().to(index_controllers::body)),
    )
    .service(
      web::scope("/todo")
        .service(
          web::resource("")
            .route(web::get().to(todo_controllers::index))
            .route(web::post().to(todo_controllers::create)),
        )
        .service(
          web::resource("/{id}")
            .route(web::get().to(todo_controllers::read))
            .route(web::put().to(todo_controllers::update))
            .route(web::delete().to(todo_controllers::delete)),
        ),
    );
}
