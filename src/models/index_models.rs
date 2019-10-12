use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct PathPath {
  pub string: String,
  pub float: f64,
  pub integer: i64,
}

#[derive(Debug, Serialize)]
pub struct PathResponse {
  pub string: String,
  pub float: f64,
  pub integer: i64,
}

#[derive(Debug, Deserialize)]
pub struct QueryQuery {
  pub string: String,
  pub float: f64,
  pub integer: i64,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
  pub string: String,
  pub float: f64,
  pub integer: i64,
}

#[derive(Debug, Deserialize)]
pub struct BodyJson {
  pub string: String,
  pub float: f64,
  pub integer: i64,
}

#[derive(Debug, Serialize)]
pub struct BodyResponse {
  pub string: String,
  pub float: f64,
  pub integer: i64,
}