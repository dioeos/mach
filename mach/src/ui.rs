use crate::config::Macros;
use slint::ComponentHandle;
use slint::{ModelRc, PlatformError, VecModel};
use std::rc::Rc;

use crate::slint_generatedAppWindow::Macro as UIMacro;
use crate::slint_generatedAppWindow::AppWindow;

pub fn make_app(macros: Vec<Macros>) -> Result<AppWindow, PlatformError> {
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()));

    let ui = AppWindow::new()?;
    let wind = ui.window();

    let ui_models: Vec<UIMacro> = macros
        .into_iter()
        .map(|m| UIMacro {
            keys_ui: m.keys.into(),
            action_ui: m.action.into(),
        })
        .collect::<Vec<UIMacro>>();

    let rc_macro_model = ModelRc::from(Rc::new(VecModel::from(ui_models)));
    ui.set_macros(rc_macro_model);

    let weak = ui.as_weak();
    wind.on_close_requested(move || {
        if let Some(w) = weak.upgrade() {
            let _ = w.hide();
        }
        println!("Hiding");
        slint::CloseRequestResponse::HideWindow
    });

    Ok(ui)
}
