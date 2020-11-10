#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::cell::RefCell;

mod cmd;
mod command_error;
mod commands;

use command_error::CommandError;
use commands::IncrementPromiseResponse;

struct State {
  count: u64,
  appname: String,
}

impl State {
  pub fn new() -> Self {
    Self {
      count: 0,
      appname: "Tauri-Application".to_string(),
    }
  }
}

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      let mut state = RefCell::new(State::new());
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            Increment { payload } => {
              let mut st = state.borrow_mut();
              println! {"\n SYNCRONOUS"};
              println! {"Incrementing from {}", st.count};
              st.count += 1;
              println! {"Incremented to {}", st.count};
            }
            IncrementPromise {
              payload,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                let mut st = state.borrow_mut();
                println! {"\n A-SYNCRONOUS"};
                println! {"Incrementing Promise from {}", st.count};
                st.count += 1;
                let response = IncrementPromiseResponse {
                  newcount: st.count,
                  // newcount: payload.count + 1,
                  message: "async response!",
                };
                println! {"Incremented Promise to {}", st.count};
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
