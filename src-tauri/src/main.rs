#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::{convert::{Format, image2x}, file_metadata, Meta};
use tauri::{CustomMenuItem, Submenu, Menu, MenuItem, generate_context};

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let _menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);

  let context = generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![image2x_command,file_metadata_command])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}


#[tauri::command]
async fn image2x_command(x: Format,index:i32,source: String)->Result<i32, String> {
  match image2x(x, source).await {
    Ok(_) => Ok(index),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
async fn file_metadata_command(files: Vec<String>) -> Result<Vec<Meta>,String> {
  match file_metadata(files).await {
    Ok(data) => Ok(data),
    Err(e) => Err(e.to_string()),
}
}