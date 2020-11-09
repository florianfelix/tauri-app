use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct IncrementPromisePayload {
    pub count: u64
}

#[derive(Serialize)]
pub struct IncrementPromiseResponse<'a> {
  pub newcount: u64,
  pub message: &'a str,
}