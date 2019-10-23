use chrono::NaiveDateTime;
use mysql;
use mysql::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
pub struct Todo {
  pub id: i64,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl FromRow for Todo {
  fn from_row(row: mysql::Row) -> Self {
    Self::from_row_opt(row).expect("failed to deserialize data")
  }

  fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
    FromRow::from_row_opt(row).map(|(id, name, created_at, updated_at)| Self {
      id,
      name,
      created_at,
      updated_at,
    })
  }
}

#[derive(Debug, Deserialize)]
pub struct TodoPath {
  pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct TodoNew {
  pub name: String,
}
