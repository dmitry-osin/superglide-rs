use rdev::{listen, simulate, Event, EventType, Key};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::settings::Settings;

pub struct HookState {
    pub is_running: bool,
    pub is_capturing: bool,
    pub capture_field: Option<String>,
    pub is_simulating: bool,
    pub settings: Settings,
}

impl HookState {
    pub fn new(settings: Settings) -> Self {
        HookState {
            is_running: false,
            is_capturing: false,
            capture_field: None,
            is_simulating: false,
            settings,
        }
    }
}

pub fn start_keyboard_hook(app_handle: AppHandle, state: Arc<Mutex<HookState>>) {
    thread::spawn(move || {
        if let Err(e) = listen(move |event| {
            handle_event(event, &app_handle, &state);
        }) {
            eprintln!("Keyboard hook error: {:?}", e);
        }
    });
}

fn key_to_string(key: &Key) -> String {
    format!("{:?}", key)
}

pub fn string_to_key(s: &str) -> Option<Key> {
    match s {
        "Alt" => Some(Key::Alt),
        "AltGr" => Some(Key::AltGr),
        "Backspace" => Some(Key::Backspace),
        "CapsLock" => Some(Key::CapsLock),
        "ControlLeft" => Some(Key::ControlLeft),
        "ControlRight" => Some(Key::ControlRight),
        "Delete" => Some(Key::Delete),
        "DownArrow" => Some(Key::DownArrow),
        "End" => Some(Key::End),
        "Escape" => Some(Key::Escape),
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "Home" => Some(Key::Home),
        "Insert" => Some(Key::Insert),
        "LeftArrow" => Some(Key::LeftArrow),
        "MetaLeft" => Some(Key::MetaLeft),
        "MetaRight" => Some(Key::MetaRight),
        "NumLock" => Some(Key::NumLock),
        "PageDown" => Some(Key::PageDown),
        "PageUp" => Some(Key::PageUp),
        "Pause" => Some(Key::Pause),
        "PrintScreen" => Some(Key::PrintScreen),
        "Return" => Some(Key::Return),
        "RightArrow" => Some(Key::RightArrow),
        "ScrollLock" => Some(Key::ScrollLock),
        "ShiftLeft" => Some(Key::ShiftLeft),
        "ShiftRight" => Some(Key::ShiftRight),
        "Space" => Some(Key::Space),
        "Tab" => Some(Key::Tab),
        "UpArrow" => Some(Key::UpArrow),
        "BackQuote" => Some(Key::BackQuote),
        "Num1" => Some(Key::Num1),
        "Num2" => Some(Key::Num2),
        "Num3" => Some(Key::Num3),
        "Num4" => Some(Key::Num4),
        "Num5" => Some(Key::Num5),
        "Num6" => Some(Key::Num6),
        "Num7" => Some(Key::Num7),
        "Num8" => Some(Key::Num8),
        "Num9" => Some(Key::Num9),
        "Num0" => Some(Key::Num0),
        "Minus" => Some(Key::Minus),
        "Equal" => Some(Key::Equal),
        "KeyQ" => Some(Key::KeyQ),
        "KeyW" => Some(Key::KeyW),
        "KeyE" => Some(Key::KeyE),
        "KeyR" => Some(Key::KeyR),
        "KeyT" => Some(Key::KeyT),
        "KeyY" => Some(Key::KeyY),
        "KeyU" => Some(Key::KeyU),
        "KeyI" => Some(Key::KeyI),
        "KeyO" => Some(Key::KeyO),
        "KeyP" => Some(Key::KeyP),
        "LeftBracket" => Some(Key::LeftBracket),
        "RightBracket" => Some(Key::RightBracket),
        "KeyA" => Some(Key::KeyA),
        "KeyS" => Some(Key::KeyS),
        "KeyD" => Some(Key::KeyD),
        "KeyF" => Some(Key::KeyF),
        "KeyG" => Some(Key::KeyG),
        "KeyH" => Some(Key::KeyH),
        "KeyJ" => Some(Key::KeyJ),
        "KeyK" => Some(Key::KeyK),
        "KeyL" => Some(Key::KeyL),
        "SemiColon" => Some(Key::SemiColon),
        "Quote" => Some(Key::Quote),
        "BackSlash" => Some(Key::BackSlash),
        "IntlBackslash" => Some(Key::IntlBackslash),
        "KeyZ" => Some(Key::KeyZ),
        "KeyX" => Some(Key::KeyX),
        "KeyC" => Some(Key::KeyC),
        "KeyV" => Some(Key::KeyV),
        "KeyB" => Some(Key::KeyB),
        "KeyN" => Some(Key::KeyN),
        "KeyM" => Some(Key::KeyM),
        "Comma" => Some(Key::Comma),
        "Dot" => Some(Key::Dot),
        "Slash" => Some(Key::Slash),
        "KpReturn" => Some(Key::KpReturn),
        "KpMinus" => Some(Key::KpMinus),
        "KpPlus" => Some(Key::KpPlus),
        "KpMultiply" => Some(Key::KpMultiply),
        "KpDivide" => Some(Key::KpDivide),
        "Kp0" => Some(Key::Kp0),
        "Kp1" => Some(Key::Kp1),
        "Kp2" => Some(Key::Kp2),
        "Kp3" => Some(Key::Kp3),
        "Kp4" => Some(Key::Kp4),
        "Kp5" => Some(Key::Kp5),
        "Kp6" => Some(Key::Kp6),
        "Kp7" => Some(Key::Kp7),
        "Kp8" => Some(Key::Kp8),
        "Kp9" => Some(Key::Kp9),
        "KpDelete" => Some(Key::KpDelete),
        _ => None,
    }
}

enum Action {
    Capture { field: String, key: String },
    ExecuteSuperglide(Settings),
    None,
}

fn handle_event(event: Event, app_handle: &AppHandle, state: &Arc<Mutex<HookState>>) {
    let EventType::KeyPress(key) = event.event_type else {
        return;
    };

    let key_str = key_to_string(&key);

    let action = {
        let state = state.lock().unwrap();

        if state.is_simulating {
            return;
        }

        if state.is_capturing {
            Action::Capture {
                field: state.capture_field.clone().unwrap_or_default(),
                key: key_str,
            }
        } else if state.is_running && key_str == state.settings.trigger_key {
            Action::ExecuteSuperglide(state.settings.clone())
        } else {
            Action::None
        }
    };

    match action {
        Action::Capture { field, key } => {
            {
                let mut s = state.lock().unwrap();
                s.is_capturing = false;
                s.capture_field = None;
            }
            let _ = app_handle.emit(
                "key-captured",
                serde_json::json!({ "field": field, "key": key }),
            );
        }
        Action::ExecuteSuperglide(settings) => {
            state.lock().unwrap().is_simulating = true;
            let state_clone = state.clone();
            let app_handle_clone = app_handle.clone();
            thread::spawn(move || {
                execute_superglide(&settings);
                state_clone.lock().unwrap().is_simulating = false;
                let _ = app_handle_clone.emit("superglide-triggered", ());
            });
        }
        Action::None => {}
    }
}

fn execute_superglide(settings: &Settings) {
    let Some(jump_key) = string_to_key(&settings.jump_key) else {
        eprintln!("Unknown jump key: {}", settings.jump_key);
        return;
    };

    let Some(crouch_key) = string_to_key(&settings.crouch_key) else {
        eprintln!("Unknown crouch key: {}", settings.crouch_key);
        return;
    };

    // Press and immediately release jump
    let _ = simulate(&EventType::KeyPress(jump_key.clone()));
    thread::sleep(Duration::from_micros(500));
    let _ = simulate(&EventType::KeyRelease(jump_key));

    // Wait configured delay between jump and crouch (frame-precise)
    let delay_us = ((settings.delay_ms * 1000.0) as u64).max(500);
    thread::sleep(Duration::from_micros(delay_us));

    // Press crouch, hold, then release
    let _ = simulate(&EventType::KeyPress(crouch_key.clone()));
    thread::sleep(Duration::from_millis(settings.crouch_hold_ms));
    let _ = simulate(&EventType::KeyRelease(crouch_key));
}
