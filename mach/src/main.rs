#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod ui;
mod window;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use slint::{invoke_from_event_loop, PlatformError};
use std::{thread};

slint::include_modules!();

const MACRO_FILE: &str = "macros.json";

fn main() -> Result<(), PlatformError> {

    let macros: Vec<config::Macros> =
        config::load_macros(MACRO_FILE).map_err(|e| PlatformError::from(e.to_string()))?;

    let ui = ui::make_app(macros).map_err(|e| PlatformError::from(e.to_string()))?;

    let weak_wind = ui.as_weak();
    let weak_wind2 = ui.as_weak();
    let weak_wind3 = ui.as_weak();

    for m in &macros {
        println!("{} -> {}", m.keys, m.action);
    }


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

    ui.show()?;

    invoke_from_event_loop(move || {
        let weak = weak_wind3.clone();
        if let Some(component) = weak.upgrade() {
            let host = component.window();
            window::center_window(host);
        }
    })
    .unwrap();

    slint::run_event_loop_until_quit()
}
