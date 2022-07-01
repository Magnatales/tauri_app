#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use crate::hero::{create_hero};

mod hero;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet, create_hero])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}


