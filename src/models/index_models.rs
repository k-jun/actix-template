use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathPath {
  pub f_str: String,
  pub f_flt: f64,
  pub f_int: i64,
}

#[derive(Debug, Deserialize)]
pub struct QueryQuery {
  pub f_str: String,
  pub f_flt: f64,
  pub f_int: i64,
}

#[derive(Debug, Deserialize)]
pub struct BodyJson {
  pub f_str: String,
  pub f_flt: f64,
  pub f_int: i64,
}
