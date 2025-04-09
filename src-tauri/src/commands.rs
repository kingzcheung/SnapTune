use crate::{
    convert::{heif::heif_to_x, ConvertFormat, Converter},
    error::AppError,
    quant::{jpeg::Jpeg, png::Png, Compression},
    settings::Settings,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedImage {
    size: usize,
    saved_path: PathBuf,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub(crate) async fn compress_image(
    file_path: String,
    save_dir: Option<String>,
    app_handle: tauri::AppHandle,
) -> Result<CompressedImage, AppError> {
    let store = app_handle.store("settings.json").unwrap();
    let Some(kind) = infer::get_from_path(file_path.as_str()).map_err(AppError::FileNotFound)?
    else {
        return Err(AppError::InvalidImage);
    };

    let upload_data = tokio::fs::read(file_path.as_str())
        .await
        .map_err(AppError::FileNotFound)?;
    let settings = store.get("settings").unwrap_or_default();
    dbg!(&settings);
    let settings: Settings = serde_json::from_value(settings).unwrap_or_default();

    let quality = 80;
    let data = match kind.extension() {
        "jpg" => Jpeg::compress(&upload_data, quality).map_err(AppError::Any),
        "png" => Png::compress(&upload_data, quality).map_err(AppError::Any),
        _ => Err(AppError::NotImplemented),
    }?;

    let size = data.len();

    let p = Path::new(&file_path);
    let file_name = p.file_name().unwrap().to_str().unwrap();

    let saved_path = match save_dir {
        Some(s) => Path::new(&s).join(file_name),
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
pub async fn open_folder(path: String, app_handle: tauri::AppHandle) -> Result<(), AppError> {
    let shell = app_handle.shell();
    // shell.command("open").args([path]).output().await.unwrap();
    // tauri_plugin_shell::open::open(scope, path, with)

    let _ = shell.open(path, None);
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct ConverterResponse {
    status: String,
    save_path: PathBuf,
}

#[tauri::command]
pub async fn convert(file_path: String, to_format: String) -> Result<ConverterResponse, AppError> {
    let Some(kind) = infer::get_from_path(file_path.as_str()).map_err(AppError::FileNotFound)?
    else {
        return Err(AppError::InvalidImage);
    };
    let p = Path::new(&file_path);
    let file_stem = p.file_stem().unwrap().to_str().unwrap();
    let filename = format!("{}.{}", file_stem, &to_format);
    let ext = kind.extension();

    let data = match ext {
        "jpg" | "png" | "webp" | "avif" | "bmp" | "gif" | "tiff" | "hdr" |"exr" => {
            let c: ConvertFormat = ext.try_into()?;
            let fmt: ConvertFormat = to_format.try_into()?;
            let file_data = tokio::fs::read(file_path.as_str())
                .await
                .map_err(AppError::FileNotFound)?;
            let d = c.convert(&file_data, fmt).map_err(AppError::Any)?;
            Ok(d)
        },
        "heif" =>{
            let file_data = tokio::fs::read(file_path.as_str())
                .await
                .map_err(AppError::FileNotFound)?;
            let fmt: ConvertFormat = to_format.try_into()?;
            let d = heif_to_x(&file_data, fmt).map_err(|e| AppError::Any(e))?;
            Ok(d)
        }
        _ => Err(AppError::NotImplemented),
    }?;

    let saved_path = p.parent().unwrap().join("output").join(filename);

    tokio::fs::create_dir_all(saved_path.parent().unwrap())
        .await
        .map_err(AppError::FileNotFound)?;

    tokio::fs::write(&saved_path, &data)
        .await
        .map_err(AppError::FileNotFound)?;

    Ok(ConverterResponse {
        status: "done".to_string(),
        save_path: saved_path,
    })
}

#[tauri::command]
pub async fn crop_image(
    image_path: String,
    crop_width: u32,
    crop_height: u32,
    x: u32,
    y: u32,
    save_path: String,
) -> Result<(), AppError> {
    crate::crop::crop_image(image_path, crop_width, crop_height, x, y, save_path).await
}

#[tauri::command]
pub async fn resize_image(
    image_path: String,
    width: u32,
    height: u32,
    save_path: String,
) -> Result<(), AppError> {
    crate::crop::resize_image(image_path, width, height, save_path).await
}

#[tauri::command]
pub async fn get_file_type(file_path: String) -> Result<String, AppError> {
    let Some(kind) = infer::get_from_path(file_path.as_str()).map_err(AppError::FileNotFound)?
    else {
        return Err(AppError::InvalidImage);
    };
    Ok(kind.extension().to_string())
}
