#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest;
use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello, get_manga_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    if name.contains(' ') {
        Err("Oi that's a spicy word, be careful".to_string())
    } else {
        Ok(format!("Ahoj {}", name))
    }
}

#[tauri::command]
async fn get_manga_list(tags: String, offset: i32) -> Result<String, String> {
    let resp = reqwest::get("https://api.mangadex.org/manga").await;
    match resp {
        Ok(res) => match res.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(format!("Error: {:?}", e)),
        },
        Err(e) => Err(format!("Error: {:?}", e)),
    }
}
