#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello(name:&str) -> Result<String,String> {
    if name.contains(' ') {
        Err("Oi that's a spicy word, be careful".to_string())
    } else {
        Ok(format!("Ahoj {}", name))
    }
}
