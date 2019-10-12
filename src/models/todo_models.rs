
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct TodoPath {
  pub id: String,
}

#[derive(Debug)]
pub struct Todo {
  pub id: i64,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl Todo {
  pub fn to_json(&self) -> TodoResponse {
    TodoResponse {
      id: self.id,
      name: self.name.to_string(),
      created_at: self.created_at.to_string(),
      updated_at: self.updated_at.to_string(),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct TodoResponse {
  pub id: i64,
  pub name: String,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct TodoNew {
  pub name: String,
}

#[derive(Debug, Serialize)]
pub struct TodoIDResponse {
  pub id: u64,
}