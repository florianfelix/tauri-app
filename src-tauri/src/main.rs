#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod command_error;
mod commands;

use command_error::CommandError;
use commands::IncrementPromiseResponse;


fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            Increment { payload } => {
              println! {"Incrementing {}", payload.count};
            }
            IncrementPromise {
              payload,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                println! {"Rust: Incrementing Promise {}", payload.count};
                let response = IncrementPromiseResponse {
                  newcount: payload.count + 1,
                  message: "async response!",
                };
                Ok(response)
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
