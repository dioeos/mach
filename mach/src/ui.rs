use crate::config::Macros;
use crate::search::{self, fuzzy_search};
use fuzzy_matcher::skim::SkimMatcherV2;
use slint::{ComponentHandle, FilterModel, SharedString};
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

    let rc_all_macros = Rc::new(ui_macros.clone());
    let rc_vec_model = Rc::new(VecModel::from(ui_macros));
    let vec_model_for_closure = rc_vec_model.clone();
    let rc_macro_model = ModelRc::from(rc_vec_model);
    ui.set_macros(rc_macro_model);

    let matcher = SkimMatcherV2::default();

    let weak_ui = ui.as_weak();
    let all_macros_for_closure = rc_all_macros.clone();
    ui.on_search_text_changed(move |_text| {
        if let Some(w) = weak_ui.upgrade() {
            let query = w.get_search_text().to_lowercase();

            if query.trim().is_empty() {
                vec_model_for_closure.clear();
                for m in all_macros_for_closure.iter() {
                    vec_model_for_closure.push(m.clone());
                }
                return;
            }

            let candidate_keys: Vec<SharedString> = all_macros_for_closure
                .iter()
                .map(|m| m.action_ui.clone())
                .collect();

            let top_keys: Vec<&SharedString> = fuzzy_search(&matcher, &query, &candidate_keys, 5);

            vec_model_for_closure.clear();
            for key in top_keys {
                if let Some(m) = all_macros_for_closure.iter().find(|m| m.action_ui == key) {
                    vec_model_for_closure.push(m.clone());
                }
            }
        }
    });

    let weak = ui.as_weak();
    wind.on_close_requested(move || {
        if let Some(w) = weak.upgrade() {
            let _ = w.hide();
        }
        slint::CloseRequestResponse::HideWindow
    });

    Ok(ui)
}
