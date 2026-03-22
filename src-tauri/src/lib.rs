mod capture;
mod credentials;
mod export;
mod history;
mod upload;

use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

fn focus_main_window(app: &tauri::AppHandle) {
    let _ = app.show();

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // Register the global shortcut (Cmd+Shift+A)
            // This will emit an event when triggered
            let handle = app.handle().clone();
            let hotkey = "CmdOrCtrl+Shift+A";

            match app.global_shortcut().on_shortcut(
                hotkey,
                move |_app, _shortcut, _event| {
                    // Emit event to frontend to trigger capture
                    let _ = handle.emit("trigger-capture", ());
                },
            ) {
                Ok(_) => {
                    if let Err(err) = app.global_shortcut().register(hotkey) {
                        eprintln!("Global shortcut unavailable ({hotkey}): {err}");
                    }
                }
                Err(err) => {
                    eprintln!("Failed to bind global shortcut handler ({hotkey}): {err}");
                }
            }

            focus_main_window(&app.handle().clone());
            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                capture::cleanup_temp_files();
            }
        })
        .invoke_handler(tauri::generate_handler![
            capture::capture_screenshot,
            export::export_annotated,
            history::save_to_history,
            history::get_history,
            history::delete_from_history,
            history::get_storage_usage,
            history::update_history_metadata,
            credentials::store_credential,
            credentials::get_credential,
            credentials::delete_credential,
            upload::upload_screenshot,
            upload::validate_credentials,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::Ready => focus_main_window(app_handle),
        #[cfg(target_os = "macos")]
        tauri::RunEvent::Reopen { .. } => focus_main_window(app_handle),
        _ => {}
    });
}
