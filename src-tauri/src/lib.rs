use tauri::{AppHandle, Manager, Theme, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_decorum::WebviewWindowExt;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_new_window(app: AppHandle, label: String, route: String) {
    println!("Creating a new window with label: {label}, route: {route}");

    let new_window = WebviewWindowBuilder::new(&app, label.clone(), WebviewUrl::default())
        .title("Dynamic Window")
        .resizable(true)
        .inner_size(800.0, 600.0)
        .min_inner_size(400.0, 300.0)
        .theme(Some(Theme::Light))
        .build();

    match new_window {
        Ok(win) => {
            win.create_overlay_titlebar().unwrap();
            println!("New window '{label}' created successfully.");
            if let Err(e) = win.show() {
                println!("Failed to show window '{label}': {e}");
            }
            println!("Hello");
        }
        Err(e) => {
            println!("Error creating window '{label}': {e}");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_decorum::init())
        .invoke_handler(tauri::generate_handler![greet, create_new_window])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
			main_window.create_overlay_titlebar().unwrap();
            Ok(())
        }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
