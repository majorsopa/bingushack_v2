use std::{rc::Rc, sync::{Mutex, Arc}};

use bingus_module::prelude::{BingusModule, populate_modules, BingusModuleTrait};
use eframe::egui;
use bingus_ui::module_widget;
use jni::JNIEnv;
use jni_mappings::{get_javavm, MappingsManager};

use crate::message_box;

pub struct BingusClient {
    modules: Arc<Mutex<Vec<BingusModule>>>,
}

impl BingusClient {
    pub fn new(modules: Arc<Mutex<Vec<BingusModule>>>) -> Self {
        Self {
            modules,
        }
    }
}

impl eframe::App for BingusClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("bingushack");
            ui.separator();
            for module in self.modules.lock().unwrap().iter_mut() {
                ui.add(module_widget(module));
            }
        });
    }
}

pub fn run_client() {
    let modules = Arc::new(Mutex::new(populate_modules()));
    let app = BingusClient::new(Arc::clone(&modules));

    let options = eframe::NativeOptions::default();


    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let running_modules = std::thread::spawn(move || {
        let jvm = unsafe { get_javavm() };
        let jni_env = unsafe { std::mem::transmute(jvm.attach_current_thread_as_daemon().unwrap()) };
        let mappings_manager = MappingsManager::new(jni_env);
        loop {
            for module in modules.lock().unwrap().iter_mut() {
                if module.get_enabled().0.get_bool() {
                    module.tick(jni_env, &mappings_manager);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(5));
            if rx.try_recv().is_ok() {
                break;
            }
        }
    });


    eframe::run_native("bingushack", options, Box::new(|_cc| Box::new(app)));
    tx.send(()).unwrap();
    running_modules.join().unwrap();
}
