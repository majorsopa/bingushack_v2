use bingus_setting::prelude::BingusSetting;
use eframe::egui;

use crate::{toggle, DoubleSlider};

use super::keybind_widget;

fn setting_ui(ui: &mut egui::Ui, setting: (&mut BingusSetting, &'static str, Option<[f32; 2]>)) -> egui::Response {
    ui.label(setting.1);
    match setting.0 {
        BingusSetting::BoolSetting(_) => {
            ui.add(toggle(setting.0.get_bool()))
        }
        BingusSetting::IntSetting(_) => {
            let range = setting.2.unwrap();
            let range: [u32; 2] = [range[0] as u32, range[1] as u32];
            ui.add(egui::Slider::new(setting.0.get_int(), range[0]..=range[1]))
        }
        BingusSetting::KeySetting(_) => {
            ui.add(keybind_widget(setting.0.get_key()))
        }
        BingusSetting::FloatSetting(_) => {
            let range = setting.2.unwrap();
            ui.add(egui::Slider::new(setting.0.get_float(), range[0]..=range[1]).step_by(1.0))
        }
        BingusSetting::RangeSetting(_) => {
            let range = setting.2.unwrap();
            ui.add(DoubleSlider::new(setting.0.get_range(), range[0]..=range[1]).step_by(1.0))
        }
    }
}

pub fn setting_widget<'a>(setting: (&'a mut BingusSetting, &'static str, Option<[f32; 2]>)) -> impl eframe::egui::Widget + 'a {
    move |ui: &mut egui::Ui| setting_ui(ui, setting)
}
