use bingus_module::prelude::{BingusModule, BingusModuleTrait};
use eframe::egui;

use crate::setting_widget;

fn module_ui(ui: &mut egui::Ui, module: &mut BingusModule) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 2.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        ui.horizontal(|ui| {
            ui.label(module.get_name());
            ui.add(setting_widget(module.get_enabled_mut()));
        });
    }

    response
}

pub fn module_widget(module: &mut BingusModule) -> impl eframe::egui::Widget + '_ {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}
