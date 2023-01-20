use std::sync::{Mutex, Arc};

use bingus_module::prelude::{BingusModule, populate_modules, BingusModuleTrait};
use eframe::egui;
use bingus_ui::module_widget;

use jni_mappings::{get_javavm, MappingsManager};
use winapi::{shared::windef::{HDC, HGLRC}, um::wingdi::{wglGetCurrentDC, wglGetCurrentContext, wglMakeCurrent}};

use crate::MODULES;


static mut CLICKGUI_CONTEXT: Option<HGLRC> = None;
static mut CLICKGUI_HDC: Option<HDC> = None;

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
        if unsafe { CLICKGUI_HDC.is_none() } {
            unsafe {
                CLICKGUI_HDC = Some(wglGetCurrentDC());
            }
        }
        if unsafe { CLICKGUI_CONTEXT.is_none() } {
            unsafe {
                CLICKGUI_CONTEXT = Some(wglGetCurrentContext());
            }
        }



        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("bingushack");
            ui.separator();
            for (i, module) in self.modules.lock().unwrap().iter_mut().enumerate() {
                ui.push_id(i, |ui| {
                    ui.add(module_widget(module));
                });
            }
        });



        unsafe {
            let hdc = *CLICKGUI_HDC.as_ref().unwrap();
            let context = CLICKGUI_CONTEXT.as_mut().unwrap();
            wglMakeCurrent(hdc, *context);
        }
    }
}

pub fn run_client() {
    let modules = MODULES.get_or_init(|| Arc::new(Mutex::new(populate_modules())));
    let app = BingusClient::new(Arc::clone(modules));

    let options = eframe::NativeOptions::default();


    let (modules_tx, modules_rx) = std::sync::mpsc::channel::<()>();
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

            std::thread::sleep(std::time::Duration::from_millis(1));
            if modules_rx.try_recv().is_ok() {
                break;
            }
        }
    });


    eframe::run_native("bingushack", options, Box::new(|_cc| Box::new(app)));
    modules_tx.send(()).unwrap();
    running_modules.join().unwrap();
    unsafe {
        CLICKGUI_CONTEXT = None;
        CLICKGUI_HDC = None;
    }
}
