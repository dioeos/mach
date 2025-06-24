use std::thread;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use slint::{invoke_from_event_loop, ComponentHandle, Weak};

use crate::slint_generatedAppWindow;

pub fn spawn_hotkey_listener(weak_window: slint::Weak<slint_generatedAppWindow::AppWindow>) -> GlobalHotKeyManager {
    let manager = GlobalHotKeyManager::new().expect("Failed to initialize HotKey Manager");
    let alt_slash_hk = HotKey::new(Some(Modifiers::ALT), Code::Slash);
    manager
        .register(alt_slash_hk)
        .expect("Failed to register alt_slash_hk");

    let rx = GlobalHotKeyEvent::receiver().clone();

    thread::spawn(move || {
        println!("In new thread");
        for event in rx {
            if event.state() == HotKeyState::Pressed {
                let weak = weak_window.clone();
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
    manager
}
