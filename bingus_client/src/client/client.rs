use bingus_module::prelude::{BingusModule, populate_modules};
use eframe::egui;
use bingus_ui::module_widget;

pub struct BingusClient {
    modules: Vec<BingusModule>,
}

impl BingusClient {
    pub fn new() -> Self {
        let new_self = Self {
            modules: populate_modules(),
        };

        new_self
    }
}

impl eframe::App for BingusClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("bingushack");
            ui.separator();
            ui.label("Modules:");
            ui.separator();
            for module in &mut self.modules {
                ui.add(module_widget(module));
            }
        });
    }
}

pub fn run_client() {
    let app = BingusClient::new();
    let options = eframe::NativeOptions::default();
    eframe::run_native("bingushack", options, Box::new(|_cc| Box::new(app)));
}
