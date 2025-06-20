use anyhow::Result;
use crossbeam_channel::Receiver;
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

struct MyApp {
    window: Option<Window>,
    visible: bool,
    hotkey_rx: Option<Receiver<GlobalHotKeyEvent>>,
    _hk_manager: Option<GlobalHotKeyManager>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            window: None,
            visible: true,
            hotkey_rx: None,
            _hk_manager: None,
        }
    }
}

impl ApplicationHandler for MyApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("Mach")
            .with_inner_size(PhysicalSize::new(800, 600));

        let win = event_loop
            .create_window(attrs)
            .expect("Failed to create window");

        self.window = Some(win);

        let mut manager = GlobalHotKeyManager::new().expect("hotkey init failed");
        let hk = HotKey::new(Some(Modifiers::CONTROL), Code::Space);
        manager.register(hk).expect("register failed");

        let rx: Receiver<GlobalHotKeyEvent> = GlobalHotKeyEvent::receiver().clone();
        println!("===App resumed===");
        self.hotkey_rx = Some(rx);
        self._hk_manager = Some(manager);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        println!("About to wait");
        if let Some(rx) = &self.hotkey_rx {
            while let Ok(evt) = rx.try_recv() {
                if evt.state == HotKeyState::Pressed {
                    self.visible = !self.visible;
                    if let Some(win) = &self.window {
                        win.set_visible(self.visible);
                    }
                }
            }
        }

        // if self.visible {
        //     if let Some(win) = &self.window {
        //         println!("Triggered - in plain");
        //         win.request_redraw();
        //     }
        // }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Exiting");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                println!("Resizing");
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw");
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let mut my_app = MyApp::new();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut my_app)?;
    Ok(())
}
