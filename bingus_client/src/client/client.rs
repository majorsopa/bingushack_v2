use std::{sync::{Mutex, Arc, atomic::AtomicPtr}, collections::HashMap, borrow::BorrowMut};

use bingus_module::prelude::{BingusModule, populate_modules, BingusModuleTrait};
use eframe::egui;
use bingus_ui::module_widget;

use jni_mappings::{get_javavm, MappingsManager};
use winapi::{shared::windef::{HDC, HGLRC}, um::{wingdi::{wglGetCurrentDC, wglGetCurrentContext, wglMakeCurrent}, winuser::GetAsyncKeyState}};

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
            ui.label("for keybinds,");
            ui.hyperlink("https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes");
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

    let mut options = eframe::NativeOptions::default();

    options.resizable = false;
    options.drag_and_drop_support = false;

    let (modules_tx, modules_rx) = std::sync::mpsc::channel::<()>();
    let running_modules = std::thread::spawn(move || {
        let jvm = unsafe { get_javavm() };
        let jni_env = unsafe { std::mem::transmute(jvm.attach_current_thread_as_daemon().unwrap()) };
        let mappings_manager = Arc::new(MappingsManager::new(jni_env));
        for module in modules.lock().unwrap().iter_mut() {
            module.init(jni_env, &mut Arc::clone(&mappings_manager));
        }
        loop {
            let mut keys_multimap: HashMap<i32, Vec<usize>> = HashMap::new();
            let mut enabled_modules_map: HashMap<usize, bool> = HashMap::new();
            for (i, module) in modules.lock().unwrap().iter_mut().enumerate() {
                enabled_modules_map.insert(i, *module.get_enabled().0.get_bool());
                if let Ok(key) = {
                    let key = module.get_keybind().0.get_key();
                    let prefix_removed = key.trim_start_matches("0x");
                    u32::from_str_radix(prefix_removed, 16)
                } {
                    keys_multimap.entry(key as i32).or_insert_with(Vec::new).push(i);
                }

                if *module.get_enabled().0.get_bool() {
                    module.tick();
                }
            }
            for (key, modules_vec) in keys_multimap {
                if unsafe { GetAsyncKeyState(key) } & 0x01 == 1 {
                    for i in modules_vec {
                        modules.lock().unwrap()[i].toggle();
                    }
                }
            }
            for (i, enabled) in enabled_modules_map {
                let mut locked_inner_modules = modules.lock().unwrap();
                let inner_module = locked_inner_modules.get_mut(i).unwrap();
                if enabled != *inner_module.get_enabled().0.get_bool() {
                    if enabled {
                        inner_module.on_enable();
                    } else {
                        inner_module.on_disable();
                    }
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
