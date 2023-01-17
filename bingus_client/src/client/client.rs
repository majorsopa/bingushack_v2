use std::rc::Rc;

use bingus_module::prelude::{BingusModule, populate_modules, BingusModuleTrait};
use eframe::egui;
use bingus_ui::module_widget;
use jni::{JNIEnv, JavaVM};
use jni_mappings::{get_javavm, MappingsManager};

pub struct BingusClient {
    modules: Vec<BingusModule>,
    jvm: JavaVM,  // idk if this is needed
    env: JNIEnv<'static>,
    mappings_manager: Rc<MappingsManager<'static>>,
}

impl BingusClient {
    pub fn new() -> Self {
        let jvm = unsafe { get_javavm() };
        let env = jvm.attach_current_thread_as_daemon().unwrap();
        let env: JNIEnv<'static> = unsafe { std::mem::transmute(env) };
        let new_self = Self {
            modules: populate_modules(),
            jvm,
            env,
            mappings_manager: Rc::new(MappingsManager::new(env)),
        };

        new_self
    }
}

impl eframe::App for BingusClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("bingushack");
            ui.separator();
            for module in &mut self.modules {
                ui.add(module_widget(module));
            }
        });

        for module in &mut self.modules {
            module.tick(self.env, Rc::clone(&self.mappings_manager));
        }
    }
}

pub fn run_client() {
    let app = BingusClient::new();
    let options = eframe::NativeOptions::default();
    eframe::run_native("bingushack", options, Box::new(|_cc| Box::new(app)));
}
