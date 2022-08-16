#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{io::Write};

use app::{
    convert::{image2x, Format},
    file_metadata,
    yolo::Yolov5,
    Meta,
};
use chrono::Local;
use image::EncodableLayout;
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use tauri::{generate_context, CustomMenuItem, Menu, MenuItem, Submenu};

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
        .invoke_handler(tauri::generate_handler![
            image2x_command,
            file_metadata_command,
            detection_command,
        ])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
async fn image2x_command(
    index: i32,
    output_format: Format,
    source: String,
    source_format: Format,
) -> Result<i32, String> {
    match image2x(output_format, source.as_str(), source_format).await {
        Ok(_) => Ok(index),
        Err(e) => Err(format!("转换错误:{}", e)),
    }
}

#[tauri::command]
async fn file_metadata_command(files: Vec<String>) -> Result<Vec<Meta>, String> {
    match file_metadata(files).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
enum MessageType {
    Info,
    Success,
    Error,
    Warning,
}

#[derive(serde::Serialize)]
struct DetectionResponse {
  message: String,
  extra: Vec<String>,
  msg_type: MessageType
}

#[tauri::command]
async fn detection_command(onnx_file: String, from: String, to: String) -> Result<DetectionResponse, String> {
    let mut yolo = Yolov5::new(onnx_file).map_err(|_| "加载 onnx 错误".to_string())?;

    let conf_thresh = 0.5;
    let nms_thresh = 0.5;

    let mat = imread(from.as_str(), IMREAD_COLOR).map_err(|_| "加载图片错误".to_string())?;
    match yolo.detect(&mat, conf_thresh, nms_thresh) {
        Ok(dets) => {
            let mut i = 0;
            let mut message = String::new();
            let mut msg_type = MessageType::Info;
            let mut extra = vec![];
            if dets.detections.is_empty() {
              message.push_str(format!("未检测到有商品: {}",from).as_str());
            }
            for det in dets.detections {
              
                let s = app::yolo::crop(&mat, det.xmin, det.ymin, det.xmax, det.ymax);
                match s {
                    Ok(img) => {
                        let bytes =
                            app::yolo::encode(&img).map_err(|_| "转换图片数据错误".to_string())?;
                      
                        let p = std::path::Path::new(from.as_str()).extension().unwrap();
                        let now = Local::now();
                        let path = format!("{}/{}_{}_{}.{}", to, det.class, now.timestamp_millis(), i,p.to_str().unwrap_or("jpg"));
                        let mut file = std::fs::File::create(path.clone()).unwrap();
                        let _ = file.write(bytes.as_bytes());
                      message.push_str("识别成功\n");
                      extra.push(path);
                      msg_type = MessageType::Success;
                    }
                    Err(e) => {
                      message.push_str(&e.to_string());
                      msg_type = MessageType::Warning;
                      continue;
                    },
                }
                i += 1;
            }
            
            Ok(DetectionResponse{ message, extra, msg_type })
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
