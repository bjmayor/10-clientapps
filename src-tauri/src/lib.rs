// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod commands;
mod utils;

use commands::{get_app_dir, greet};
use tauri::{webview, App, Webview, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent};
use tauri_plugin_log::{Target, TargetKind};
use tracing::{debug, info};
use utils::log_dir;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");
    let handle = app.handle();
    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
    #[cfg(desktop)]
    {
        builder = builder
            .title("Hacker News")
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }

    let webview = builder.build()?;

    #[cfg(debug_assertions)]
    {
        webview.open_devtools();
    }
    Ok(())
}

fn page_load_handler(_window: &Webview, payload: &webview::PageLoadPayload<'_>) {
    info!("Page loaded: {:?}", payload.url());
}

fn window_event_handler(window: &Window, event: &tauri::WindowEvent) {
    debug!("Window event: {:?}", event);
    if let WindowEvent::CloseRequested { api, .. } = event {
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap()
        }
    }
}
fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}
