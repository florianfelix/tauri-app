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

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}
