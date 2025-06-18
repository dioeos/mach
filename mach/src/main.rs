use eframe::{
    self, App, CreationContext, Frame, NativeOptions, WindowBuilderHook,
    egui::{CentralPanel, ComboBox, PopupCloseBehavior, UiKind::Popup, Vec2, X11WindowType},
};

use egui::{UiKind, ViewportBuilder, Window, viewport::WindowLevel};
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};

use winit::window::WindowBuilder;

use crossbeam_channel::Receiver;

struct MyApp {
    hotkey_rx: Receiver<GlobalHotKeyEvent>,
}

impl MyApp {
    fn new(hotkey_rx: Receiver<GlobalHotKeyEvent>) -> Self {
        Self { hotkey_rx }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if let Ok(event) = self.hotkey_rx.try_recv() {
            if event.id() == HotKey::new(Some(Modifiers::ALT), Code::Slash).id()
                && event.state() == HotKeyState::Pressed
            {
                println!("Alt+N pressed");
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hotkey + egui Demo");
            ui.label("Press Alt + Slash anywhere to see the message in the console.");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let manager = GlobalHotKeyManager::new().unwrap();
    let hk = HotKey::new(Some(Modifiers::ALT), Code::Slash);
    manager.register(hk).unwrap();

    println!("Registered hotkeyts... launching GUI now!");

    let hotkey_rx: Receiver<GlobalHotKeyEvent> = GlobalHotKeyEvent::receiver().clone();

    let viewport = ViewportBuilder {
        inner_size: Some(Vec2::new(800.0, 600.0)),
        min_inner_size: Some(Vec2::new(800.0, 600.0)),
        max_inner_size: Some(Vec2::new(800.0, 600.0)),
        resizable: Some(false),
        fullscreen: Some(false),
        maximized: Some(false),
        decorations: Some(true),
        window_type: Some(X11WindowType::PopupMenu),
        window_level: Some(WindowLevel::AlwaysOnTop),
        ..Default::default()
    };

    let options = NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Mach",
        options,
        Box::new(move |_cc: &CreationContext<'_>| Ok(Box::new(MyApp::new(hotkey_rx.clone())))),
    )
}
