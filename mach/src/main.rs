#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use slint::{invoke_from_event_loop, CloseRequestResponse, PlatformError};
use std::{
    error::Error,
    io::{self, ErrorKind},
    path::PathBuf,
    thread,
};

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?; //component
    let wind = ui.window(); //window componenet
    let weak_wind = ui.as_weak();
    let weak_wind2 = ui.as_weak();

    let macro_file = "macros.json";

    let file_path: PathBuf =
        file::get_config_file(macro_file).map_err(|e| PlatformError::from(e.to_string()))?;

    let macros: Vec<file::Macros> = match file::read_macros(&file_path) {
        Ok(m) => {
            println!("Received");
            m
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("Not found, defaults");
            let defaults = file::write_default_macros(&file_path)
                .map_err(|e| PlatformError::from(e.to_string()))?;
            defaults
        }
        Err(e) => {
            return Err(PlatformError::from(format!("I/O error: {}", e)));
        }
    };

    for m in &macros {
        println!("{} -> {}", m.keys, m.action);
    }

    wind.on_close_requested(move || {
        if let Some(w) = weak_wind.upgrade() {
            let _ = w.hide();
        }
        println!("Hiding");
        CloseRequestResponse::HideWindow
    });
    // ui.show().unwrap();

    let manager = GlobalHotKeyManager::new().unwrap();
    let hk = HotKey::new(Some(Modifiers::ALT), Code::Slash);

    manager.register(hk).expect("Failed to register hotkey");

    let rx = GlobalHotKeyEvent::receiver().clone();

    thread::spawn(move || {
        println!("In new thread");
        for event in rx {
            if event.state() == HotKeyState::Pressed {
                let weak = weak_wind2.clone();
                invoke_from_event_loop(move || {
                    if let Some(component) = weak.upgrade() {
                        let host = component.window();
                        if host.is_visible() {
                            println!("Hiding");
                            host.hide();
                        } else {
                            println!("Showing");
                            host.show();
                        }
                    }
                })
                .unwrap();
            }
        }
    });

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    slint::run_event_loop_until_quit()
}
