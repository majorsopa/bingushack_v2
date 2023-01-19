use bingus_setting::prelude::BingusSetting;
use eframe::egui;

use crate::toggle;

fn setting_ui(ui: &mut egui::Ui, setting: &mut (BingusSetting, &'static str)) -> egui::Response {
    ui.label(setting.1);
    match setting.0 {
        BingusSetting::BoolSetting(_) => {
            ui.add(toggle(setting.0.get_value_mut().into()))
        }
        BingusSetting::IntSetting(_) => {
            ui.add(egui::Slider::new(setting.0.get_value_mut().into(), 0..=24))
        }
    }
}

pub fn setting_widget<'a>(setting: &'a mut (BingusSetting, &'static str)) -> impl eframe::egui::Widget + 'a {
    move |ui: &mut egui::Ui| setting_ui(ui, setting)
}
