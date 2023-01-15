use bingus_module::prelude::{BingusModule, BingusModuleTrait};
use eframe::egui;

fn module_ui(ui: &mut egui::Ui, module: &mut BingusModule) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 2.0);
    let (_rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    ui.label("Module UI");
    ui.label(format!("{}", module.get_name()));

    response
}

pub fn module_widget(module: &mut BingusModule) -> impl eframe::egui::Widget + '_ {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}
