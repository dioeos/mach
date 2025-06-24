#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod window;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use slint::{invoke_from_event_loop, CloseRequestResponse, ModelRc, PlatformError, VecModel};
use std::{rc::Rc, thread};

slint::include_modules!();
use slint_generatedAppWindow::Macro as UIMacro;

const MACRO_FILE: &str = "macros.json";

fn main() -> Result<(), PlatformError> {
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()));
    let ui = AppWindow::new()?; //component
    let wind = ui.window(); //window componenet
    let weak_wind = ui.as_weak();
    let weak_wind2 = ui.as_weak();
    let weak_wind3 = ui.as_weak();

    let macros: Vec<config::Macros> =
        config::load_macros(MACRO_FILE).map_err(|e| PlatformError::from(e.to_string()))?;

    for m in &macros {
        println!("{} -> {}", m.keys, m.action);
    }

    let ui_macros: Vec<UIMacro> = macros
        .into_iter()
        .map(|m| UIMacro {
            keys_ui: m.keys.into(), //shared string
            action_ui: m.action.into(),
        })
        .collect();

    let macro_model: Rc<VecModel<UIMacro>> = Rc::new(VecModel::from(ui_macros));
    let rc_macro_model = ModelRc::from(macro_model.clone());
    ui.set_macros(rc_macro_model);

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
