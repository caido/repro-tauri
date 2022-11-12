use tauri::window::Window;
use tauri::window::WindowBuilder;
use tauri::WindowUrl;
use tauri::{command, AppHandle};

#[command]
pub async fn create_window(window: Window, app_handle: AppHandle) -> Result<(), String> {
    let url = WindowUrl::App("/page2.html".into());
    let new_window = WindowBuilder::new(&app_handle, "window-2", url)
        .title("Window 2")
        .maximized(true)
        .build()
        .unwrap();

    let _ = new_window.show();

    let _ = window.hide();

    Ok(())
}
