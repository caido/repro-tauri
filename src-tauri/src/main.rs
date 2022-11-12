#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::window::WindowBuilder;
use tauri::WindowUrl;

mod handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handler::create_window])
        .setup(|app| {
            let url = WindowUrl::App("/page1.html".into());
            let window = WindowBuilder::new(app, "window-1", url)
                .title("Window 1")
                .visible(false)
                .resizable(false)
                .inner_size(800.0, 600.0)
                .build()?;
            window.show()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
