use tauri::{ WebviewUrl, WebviewWindowBuilder};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

pub mod quant;
pub mod convert;
mod commands;
pub mod error;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Image2X")
                .inner_size(1000.0, 700.0);
            // 仅在 macOS 时设置透明标题栏
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build().unwrap();

            // 仅在构建 macOS 时设置背景颜色
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id, nil};

                let ns_window = window.ns_window().unwrap() as id;
                unsafe {
                    let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                        nil,
                        243.0 / 255.0,
                        243.0 / 255.0,
                        243.5 / 255.0,
                        1.0,
                    );
                    ns_window.setBackgroundColor_(bg_color);
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![commands::compress_image, commands::open_folder,commands::convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
