use std::{
    process::{Child, Command},
    thread,
};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use slint::{invoke_from_event_loop, ComponentHandle};

use crate::slint_generatedAppWindow;

pub fn spawn_hotkey_listener(
    weak_window: slint::Weak<slint_generatedAppWindow::AppWindow>,
) -> GlobalHotKeyManager {
    let manager = GlobalHotKeyManager::new().expect("Failed to initialize HotKey Manager");
    let alt_slash_hk = HotKey::new(Some(Modifiers::ALT), Code::Slash);
    let alt_comma_hk = HotKey::new(Some(Modifiers::ALT), Code::Comma);

    let alt_slash_hk_id = alt_slash_hk.id();
    let alt_comma_hk_id = alt_comma_hk.id();

    manager
        .register_all(&[alt_slash_hk, alt_comma_hk])
        .expect("Failed to register alt_slash_hk");

    let rx = GlobalHotKeyEvent::receiver().clone();

    thread::spawn(move || {
        for event in rx {
            if event.state() != HotKeyState::Pressed {
                continue;
            }

            if event.id() == alt_comma_hk_id {
                invoke_from_event_loop(move || {
                    if let Err(_err) = open_editor("C:/Windows/System32/notepad.exe") {
                        // Adding Error
                    }
                })
                .unwrap();
            }

            if event.id() == alt_slash_hk_id {
                let weak = weak_window.clone();
                invoke_from_event_loop(move || toggle_window(&weak)).unwrap();
            }
        }
    });

    manager
}

fn toggle_window(weak_window: &slint::Weak<slint_generatedAppWindow::AppWindow>) {
    if let Some(component) = weak_window.upgrade() {
        let host = component.window();
        if host.is_visible() {
            host.hide().unwrap();
        } else {
            host.show().unwrap();
        }
    }
}

fn open_editor(editor_path: &str) -> Result<Child, std::io::Error> {
    match Command::new(editor_path).spawn() {
        Ok(child_process) => Ok(child_process),
        Err(_) if editor_path != "code" => {
            let mut fallback = if cfg!(target_os = "windows") {
                let mut c = Command::new("cmd");
                c.arg("/C").arg("code");
                c
            } else {
                Command::new("code")
            };
            fallback.spawn()
        }
        Err(e) => Err(e),
    }
}
