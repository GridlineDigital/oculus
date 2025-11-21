// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lifx;
use lifx::{LifxState, LifxStatePayload, Light, Scene};
use tauri::{Manager, State};

#[tauri::command]
async fn set_api_token(
    token: String,
    state: State<'_, LifxState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state.update_token(token, &app).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_lights(
    state: State<'_, LifxState>,
    app: tauri::AppHandle,
) -> Result<Vec<Light>, String> {
    state.list_lights(&app).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_light_state(
    selector: String,
    payload: LifxStatePayload,
    state: State<'_, LifxState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state
        .set_state(&selector, payload, &app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_light(
    selector: String,
    state: State<'_, LifxState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state
        .toggle_power(&selector, &app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_scenes(
    state: State<'_, LifxState>,
    app: tauri::AppHandle,
) -> Result<Vec<Scene>, String> {
    state.list_scenes(&app).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn activate_scene(
    uuid: String,
    state: State<'_, LifxState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state
        .activate_scene(&uuid, &app)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::TrayIconBuilder;

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone()) // Fallback to app icon for now as loading from path requires more setup
                // To use the specific tray icon, we would need to bundle it or include_bytes!
                // For simplicity and robustness, let's stick to the default icon which is now the lightbulb
                // If the user specifically wants the monochrome one, we can swap it in the build process or use include_bytes
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .manage(LifxState::new())
        .invoke_handler(tauri::generate_handler![
            set_api_token,
            get_lights,
            set_light_state,
            toggle_light,
            get_scenes,
            activate_scene
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
