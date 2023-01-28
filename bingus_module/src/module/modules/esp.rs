use std::{thread::JoinHandle, sync::{Arc, Mutex, mpsc::{Receiver, Sender}}};
use once_cell::sync::OnceCell;
use pixels::{SurfaceTexture, Pixels};
use tao::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, platform::{run_return::EventLoopExtRunReturn, windows::EventLoopExtWindows}, event::Event};

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

fn render(esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {
    /*
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
    */
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
            let mut event_loop: EventLoop<()> = EventLoop::new_any_thread();
            let window = {
                let size = LogicalSize::new(256, 256);
                WindowBuilder::new()
                    .with_min_inner_size(size)
                    .with_decorations(false)
                    .with_resizable(false)
                    .with_always_on_top(true)
                    .with_maximized(true)
                    //.with_transparent(true)
                    .with_resizable(false)
                    .build(&event_loop)
                    .unwrap()
            };

            let mut pixels = {
                let size = window.inner_size();
                let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
                Pixels::new(size.width, size.height, surface_texture).unwrap()
            };

            event_loop.run_return(move |event, _, control_flow| {
                if let Event::RedrawRequested(_) = event {
                    let esp_window = ESP_WINDOW.get().unwrap().clone();
                    let rects = esp_window.rects.lock().unwrap();
                    let frame = pixels.get_frame_mut();
                    for rect in rects.iter() {
                        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                            let x = i as f64 % 256.0;
                            let y = i as f64 / 256.0;
                            if x >= rect[0] && x <= rect[0] + 10.0 && y >= rect[1] && y <= rect[1] + 10.0 {
                                pixel.copy_from_slice(&[255, 0, 0, 255]);
                            }
                        }
                    }

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
            });
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
