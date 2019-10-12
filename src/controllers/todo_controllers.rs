use actix_web::{
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse, Responder,
};

use mysql;


use super::super::AppState;
use crate::models::{index_models, todo_models};
pub fn index() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("plain/text")
    .header("X-Hdr", "sample")
    .body("data")
}

pub fn create(
  (state, json, req): (Data<AppState>, Json<todo_models::TodoNew>, HttpRequest),
) -> HttpResponse {
  let db = &state.db;
  let last_id = db
    .prep_exec(
      "INSERT INTO todo (name) VALUES (:name)",
      params! {"name" => &json.name},
    )
    .unwrap()
    .last_insert_id();

  HttpResponse::Ok()
    .content_type("application/json")
    .json(todo_models::TodoIDResponse { id: last_id })
}

pub fn read(
  (state, path, req): (Data<AppState>, Path<todo_models::TodoPath>, HttpRequest),
) -> HttpResponse {
  let db = &state.db;

  // TODO 存在しないIDに対するハンドリング

  let mut row = db
    .first_exec(
      "SELECT * FROM todo WHERE id = :id",
      params! {"id" => &path.id},
    )
    .unwrap()
    .unwrap();

  let todo = todo_models::Todo {
    id: row.take("id").unwrap(),
    name: row.take("name").unwrap(),
    created_at: row.take("create_date").unwrap(),
    updated_at: row.take("update_date").unwrap(),
  };

  HttpResponse::Ok()
    .content_type("application/json")
    .json(todo.to_json())
}

// pub fn update((path, req): (Path<index_models::PathPath>, HttpRequest)) -> HttpResponse {
//   HttpResponse::Ok()
//     .content_type("application/json")
//     .json(index_models::PathResponse {
//       string: path.string.to_string(),
//       integer: path.integer,
//       float: path.float,
//     })
// }

// pub fn delete((path, req): (Path<index_models::PathPath>, HttpRequest)) -> HttpResponse {
//   HttpResponse::Ok()
//     .content_type("application/json")
//     .json(index_models::PathResponse {
//       string: path.string.to_string(),
//       integer: path.integer,
//       float: path.float,
//     })
// }