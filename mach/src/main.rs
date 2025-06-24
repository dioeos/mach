#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod hotkey_listener;
mod ui;
mod window;

use slint::{invoke_from_event_loop, PlatformError};

slint::include_modules!();

const MACRO_FILE: &str = "macros.json";

fn main() -> Result<(), PlatformError> {
    let macros: Vec<config::Macros> =
        config::load_macros(MACRO_FILE).map_err(|e| PlatformError::from(e.to_string()))?;

    let app_ui = ui::make_app(macros).map_err(|e| PlatformError::from(e.to_string()))?;
    let weak_wind2 = app_ui.as_weak();

    app_ui.show()?;

    let manager = hotkey_listener::spawn_hotkey_listener(app_ui.as_weak()); //ensures manager lifetime 

    invoke_from_event_loop(move || {
        let weak = weak_wind2.clone();
        if let Some(component) = weak.upgrade() {
            let host = component.window();
            window::center_window(host);
        }
    })
    .unwrap();

    slint::run_event_loop_until_quit()
}
