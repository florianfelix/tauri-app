#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod command_error;
mod commands;

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
                let res = payload.count + 1 as u64;
                let response = IncrementPromiseResponse {
                  newcount: res,
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
