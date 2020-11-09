use serde::Deserialize;

use crate::commands::{IncrementPayload, IncrementPromisePayload};

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  Increment {
    payload: IncrementPayload,
  },
  IncrementPromise {
    payload: IncrementPromisePayload,
    callback: String,
    error: String,
  },
}


