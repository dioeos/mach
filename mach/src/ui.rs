use std::rc::Rc;
use slint::{PlatformError, VecModel, ModelRc};

slint::include_modules!();

pub fn make_app(macros: Vec<crate::config::Macros) -> Result<slint_generatedAppWindow::AppWindow, PlatformError> {
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()));

    let ui = AppWindow::new()?;
    let wind = ui.window();

    let ui_models = macros.into_iter()
        .map(|m| slint_generatedAppWindow::Macro {
            keys_ui: m.keys.into(),
            action_ui: m.action.into(),
        })
        .collect();

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
