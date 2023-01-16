use bingus_setting::prelude::{BingusSetting, BingusSettingTrait};
use eframe::egui;

use crate::toggle;

fn setting_ui(ui: &mut egui::Ui, setting: &mut BingusSetting) -> egui::Response {
    match setting {
        BingusSetting::BoolSetting(value) => {
            ui.add(toggle(value.get_value_mut().into()))
        }
        BingusSetting::IntSetting(value) => {
            ui.add(egui::Slider::new(value.get_value_mut(), 0..=24))
        }
    }
}

pub fn setting_widget(setting: &mut BingusSetting) -> impl eframe::egui::Widget + '_ {
    move |ui: &mut egui::Ui| setting_ui(ui, setting)
}
