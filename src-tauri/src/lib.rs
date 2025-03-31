mod keyboard_hook;
mod settings;

use keyboard_hook::{start_keyboard_hook, HookState};
use settings::Settings;
use std::sync::{Arc, Mutex};
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State,
};

type AppState = Arc<Mutex<HookState>>;

fn make_tray_icon(active: bool) -> Image<'static> {
    const SIZE: u32 = 32;
    let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];
    let cx = SIZE as f32 / 2.0;
    let cy = SIZE as f32 / 2.0;
    let r = SIZE as f32 / 2.0 - 4.0;

    let (rv, gv, bv): (u8, u8, u8) = if active { (230, 57, 70) } else { (90, 90, 95) };

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            if (dx * dx + dy * dy).sqrt() < r {
                let i = ((y * SIZE + x) * 4) as usize;
                pixels[i] = rv;
                pixels[i + 1] = gv;
                pixels[i + 2] = bv;
                pixels[i + 3] = 255;
            }
        }
    }

    Image::new_owned(pixels, SIZE, SIZE)
}

fn update_tray(app: &AppHandle, active: bool) {
    if let Some(tray) = app.tray_by_id("sg-tray") {
        let _ = tray.set_icon(Some(make_tray_icon(active)));
        let tooltip = if active {
            "SuperGlide RS — Active"
        } else {
            "SuperGlide RS"
        };
        let _ = tray.set_tooltip(Some(tooltip));
    }
}

#[tauri::command]
fn start_macro(state: State<AppState>, app: AppHandle) {
    state.lock().unwrap().is_running = true;
    update_tray(&app, true);
}

#[tauri::command]
fn stop_macro(state: State<AppState>, app: AppHandle) {
    state.lock().unwrap().is_running = false;
    update_tray(&app, false);
}

#[tauri::command]
fn start_capture(field: String, state: State<AppState>) {
    let mut s = state.lock().unwrap();
    s.is_capturing = true;
    s.capture_field = Some(field);
}

#[tauri::command]
fn cancel_capture(state: State<AppState>) {
    let mut s = state.lock().unwrap();
    s.is_capturing = false;
    s.capture_field = None;
}

#[tauri::command]
fn load_settings() -> Settings {
    settings::load_settings()
}

#[tauri::command]
fn save_settings(settings: Settings, state: State<AppState>) -> Result<(), String> {
    state.lock().unwrap().settings = settings.clone();
    settings::save_settings(&settings)
}

#[tauri::command]
fn hide_window(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}

#[tauri::command]
fn open_url(url: String) {
    let _ = open::that_in_background(url);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_settings = settings::load_settings();
    let hook_state: AppState = Arc::new(Mutex::new(HookState::new(initial_settings)));
    let hook_state_for_setup = hook_state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .manage(hook_state)
        .setup(|app| {
            // Start global keyboard hook
            let handle = app.handle().clone();
            start_keyboard_hook(handle, hook_state_for_setup);

            let show_item = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let sep = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&sep)
                .item(&quit_item)
                .build()?;

            TrayIconBuilder::with_id("sg-tray")
                .icon(make_tray_icon(false))
                .tooltip("SuperGlide RS")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        let _ = w.hide();
                        api.prevent_close();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_macro,
            stop_macro,
            start_capture,
            cancel_capture,
            load_settings,
            save_settings,
            hide_window,
            open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
