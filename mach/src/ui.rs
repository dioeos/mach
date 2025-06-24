use crate::config::Macros;
use slint::{ComponentHandle, FilterModel};
use slint::{ModelRc, PlatformError, VecModel};
use std::rc::Rc;

use crate::slint_generatedAppWindow::AppWindow;
use crate::slint_generatedAppWindow::Macro as UIMacro;

pub fn make_app(macros: Vec<Macros>) -> Result<AppWindow, PlatformError> {
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()));

    let ui = AppWindow::new()?;
    let wind = ui.window();

    let ui_macros: Vec<UIMacro> = macros
        .into_iter()
        .map(|m| UIMacro {
            keys_ui: m.keys.into(),
            action_ui: m.action.into(),
        })
        .collect::<Vec<UIMacro>>();

    let rc_macro_model = ModelRc::from(Rc::new(VecModel::from(ui_macros)));

    let weak_ui_for_filter = ui.as_weak();

    let raw_filtered_model = Rc::new(FilterModel::new(
        rc_macro_model.clone(),
        move |ui_macro: &UIMacro| {
            if let Some(w) = weak_ui_for_filter.upgrade() {
                let query = w.get_search_text().to_lowercase();
                ui_macro.keys_ui.to_lowercase().contains(&query)
                    || ui_macro.action_ui.to_lowercase().contains(&query)
            } else {
                true
            }
        },
    ));

    let filtered_model = ModelRc::from(raw_filtered_model.clone());

    ui.set_macros(filtered_model.clone());
    ui.on_search_text_changed(move |_text| raw_filtered_model.reset());

    let weak = ui.as_weak();
    wind.on_close_requested(move || {
        if let Some(w) = weak.upgrade() {
            let _ = w.hide();
        }
        slint::CloseRequestResponse::HideWindow
    });

    Ok(ui)
}
