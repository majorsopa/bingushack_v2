use std::{thread::JoinHandle, sync::{Arc, Mutex, mpsc::{Receiver, Sender}}};
use winit::platform::windows::EventLoopBuilderExtWindows;
use once_cell::sync::OnceCell;
use winapi::{shared::windef::{HDC, HGLRC}, um::wingdi::{wglGetCurrentDC, wglGetCurrentContext}};

use crate::crate_prelude::*;


fn tick(esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };
    let world = match get_world_checked(env, mappings_manager, minecraft_client) {
        Some(world) => world,
        None => return,
    };

    let entities_iterator = mappings_manager.get("Iterator").unwrap();
    apply_object!(
        entities_iterator,
        java_iterable_to_iterator(
            env,
            mappings_manager,
            {
                let iterable = mappings_manager.get("Iterable").unwrap();
                apply_object!(
                    iterable,
                    call_method_or_get_field!(
                        env,
                        world,
                        "getEntities",
                        false,
                        &[]
                    ).unwrap().l().unwrap()
                );
                iterable
            }
        ).get_object().unwrap()
    );

    esp.entity_list = vec![];
    loop {
        if let Some(entity) = get_next_java_iterator_checked(env, mappings_manager, entities_iterator) {  // entity is a generic Object ClassMapping, which is casted to an Entity ClassMapping
            let entity = {
                let _entity = entity;
                let entity = mappings_manager.get("Entity").unwrap();
                apply_object!(entity, _entity.get_object().unwrap());
                entity
            };

            if {
                env.is_same_object(entity.get_object().unwrap(), JObject::null()).unwrap() ||
                !is_instance_of(env, entity, mappings_manager.get("LivingEntity").unwrap()) ||
                get_entity_id(env, entity) == get_entity_id(env, {
                    let entity = mappings_manager.get("Entity").unwrap();
                    apply_object!(entity, player.get_object().unwrap());
                    entity
                })
            } {
                continue;
            }

            esp.entity_list.push(RenderInfo::new_from_entity(env, mappings_manager, entity));
        } else {
            break;
        }
    }
}



static mut ESP_JOINHANDLE: Option<JoinHandle<()>> = None;
static mut ESP_WINDOW: OnceCell<EspWindow> = OnceCell::new();
static mut ESP_CONTEXT: Option<HGLRC> = None;
static mut ESP_HDC: Option<HDC> = None;

struct EspWindow {
    rects: Arc<Mutex<Vec<[f64; 2]>>>,
    receiver: Arc<Option<Receiver<()>>>,
}

impl Clone for EspWindow {
    fn clone(&self) -> Self {
        Self {
            rects: Arc::clone(&self.rects),
            receiver: Arc::clone(&self.receiver)
        }
    }
}

impl eframe::App for EspWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if self.receiver.is_some() {
            frame.close();
            return;
        }
        if unsafe { ESP_HDC.is_none() } {
            unsafe {
                ESP_HDC = Some(wglGetCurrentDC());
            }
        }
        if unsafe { ESP_CONTEXT.is_none() } {
            unsafe {
                ESP_CONTEXT = Some(wglGetCurrentContext());
            }
        }

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("bruh");
        });
    }
}

fn render(esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {
    unsafe {
        if ESP_JOINHANDLE.is_none() {
            return;
        }
    }


    let minecraft_client = get_minecraft_client(env, mappings_manager);
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };


    let esp_window = unsafe { ESP_WINDOW.get().unwrap().clone() };
    let mut rects = esp_window.rects.lock().unwrap();
    rects.clear();
    for entity in &esp.entity_list {
        //let bounding_box = entity.get_bounding_box();  // minx miny minz maxx maxy maxz
        /*let box_vertices: [[f64; 3]; 8] = [
            [bounding_box[0] - 0.1, bounding_box[1] - 0.1, bounding_box[2] - 0.1],
            [bounding_box[0] - 0.1, bounding_box[4] + 0.1, bounding_box[2] - 0.1],
            [bounding_box[3] + 0.1, bounding_box[4] + 0.1, bounding_box[2] - 0.1],
            [bounding_box[3] + 0.1, bounding_box[1] - 0.1, bounding_box[2] - 0.1],
            [bounding_box[3] + 0.1, bounding_box[4] + 0.1, bounding_box[5] + 0.1],
            [bounding_box[0] - 0.1, bounding_box[4] + 0.1, bounding_box[5] + 0.1],
            [bounding_box[0] - 0.1, bounding_box[1] - 0.1, bounding_box[5] + 0.1],
            [bounding_box[3] + 0.1, bounding_box[1] - 0.1, bounding_box[5] + 0.1]
        ];*/

        rects.push(world_to_screen(env, mappings_manager, player, entity.entity_pos));
    }
}

fn on_disable(esp: &mut Esp) {
    esp.sender.send(()).unwrap();
    unsafe {
        ESP_JOINHANDLE.take().unwrap().join().unwrap();
    }
}

fn on_enable() {
    unsafe {
        ESP_JOINHANDLE = Some(std::thread::spawn(|| {
            let mut options = eframe::NativeOptions::default();
            options.always_on_top = true;
            options.maximized = true;
            options.decorated = false;
            options.resizable = false;
            options.transparent = true;
            options.mouse_passthrough = true;
            options.run_and_return = true;
            options.event_loop_builder = Some(Box::new(|builder|{
                builder.with_any_thread(true);
            }));
            eframe::run_native("esp", options, Box::new(|_cc| Box::new(ESP_WINDOW.get().unwrap().clone())));
        }));
    }
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(
    name = "ESP (doesn't work)",
    tick_method = "tick(self, _env, _mappings_manager)",
    render_method = "render(self, _env, _mappings_manager)",
    on_disable_method = "on_disable(self)",
    on_enable_method = "on_enable()"
)]
pub struct Esp {
    entity_list: Vec<RenderInfo>,
    sender: Sender<()>,
}

impl MakeNewBingusModule for Esp {
    fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        unsafe { ESP_WINDOW.get_or_init(|| {
            EspWindow {
                rects: Arc::new(Mutex::new(vec![])),
                receiver: Arc::new(Some(receiver))
            }
        })};
        Self {
            entity_list: vec![],
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            sender,
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}
