use std::path::{Path, PathBuf};
use tauri_plugin_shell::ShellExt;
use quant::{jpeg::Jpeg, png::Png, Compression};
use serde::{Deserialize, Serialize};

pub mod quant;
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("invalid image")]
    InvalidImage,
    #[error("not implemented")]
    NotImplemented,
    #[error("{0}")]
    Any(anyhow::Error),
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedImage {
    size: usize,
    saved_path: PathBuf,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn compress_image(
    file_path: String,
    save_path: Option<String>,
) -> Result<CompressedImage, AppError> {
    let Some(kind) = infer::get_from_path(file_path.as_str()).map_err(AppError::FileNotFound)?
    else {
        return Err(AppError::InvalidImage);
    };

    let upload_data = tokio::fs::read(file_path.as_str())
        .await
        .map_err(AppError::FileNotFound)?;
    let quality = 80;
    let data = match kind.extension() {
        "jpg" => Jpeg::compress(&upload_data, quality).map_err(AppError::Any),
        "png" => Png::compress(&upload_data, quality).map_err(AppError::Any),
        _ => Err(AppError::NotImplemented),
    }?;

    let size = data.len();

    let p = Path::new(&file_path);
    let file_name = p.file_name().unwrap().to_str().unwrap();

    let saved_path = match save_path {
        Some(s) => PathBuf::from(s),
        None => p.parent().unwrap().join("output").join(file_name),
    };

    tokio::fs::create_dir_all(saved_path.parent().unwrap())
        .await
        .map_err(AppError::FileNotFound)?;

    tokio::fs::write(&saved_path, &data)
        .await
        .map_err(AppError::FileNotFound)?;

    Ok(CompressedImage { size, saved_path })
}

#[tauri::command]
async fn open_folder(path: String,app_handle: tauri::AppHandle) -> Result<(), AppError> {
    let shell = app_handle.shell();
    // shell.command("open").args([path]).output().await.unwrap();
    // tauri_plugin_shell::open::open(scope, path, with)
    let _ = shell.open(path, None);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![compress_image,open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
