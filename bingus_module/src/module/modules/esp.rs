use std::{thread::JoinHandle, sync::{Arc, mpsc::{Receiver, Sender}}};
use once_cell::sync::OnceCell;
use pixels::{SurfaceTexture, Pixels};
use tao::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::{LogicalSize}, platform::{run_return::EventLoopExtRunReturn, windows::EventLoopExtWindows}, event::Event};
use winapi::{shared::windef::{HGLRC, HDC}, um::{wingdi::{wglGetCurrentContext, wglGetCurrentDC, wglMakeCurrent, GetDeviceCaps, LOGPIXELSX, LOGPIXELSY}}};

use crate::crate_prelude::*;



static mut ESP_CONTEXT: Option<HGLRC> = None;
static mut ESP_HDC: Option<HDC> = None;

static DIMENSIONS: OnceCell<[i32; 2]> = OnceCell::new();


fn tick(_esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {
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


    let mut entity_list = vec![];
    loop {
        if let Some(entity) = get_next_java_iterator_checked(env, mappings_manager, entities_iterator) {  // entity is a generic Object ClassMapping, which is casted to an Entity ClassMapping
            let entity = {
                let _entity = entity;
                let entity = mappings_manager.get("Entity").unwrap();
                apply_object!(entity, _entity.get_object().unwrap());
                entity
            };

            if env.is_same_object(entity.get_object().unwrap(), JObject::null()).unwrap() ||
            !is_instance_of(env, entity, mappings_manager.get("LivingEntity").unwrap()) ||
            get_entity_id(env, entity) == get_entity_id(env, {
                let entity = mappings_manager.get("Entity").unwrap();
                apply_object!(entity, player.get_object().unwrap());
                entity
            }) {
                continue;
            }

            entity_list.push(RenderInfo::new_from_entity(env, mappings_manager, entity));
        } else {
            break;
        }
    }

    let esp_window = unsafe { ESP_WINDOW.get().unwrap().clone() };
    let mut rects = esp_window.rects.lock().unwrap();
    // IMPORTANT
    if !rects.is_empty() {
        return;
    }
    for entity in entity_list {
        rects.push(world_to_screen(entity.entity_pos));
    }
    send_chat_message(env, mappings_manager, player, &format!("{rects:#?}"));
}



static mut ESP_JOINHANDLE: Option<JoinHandle<()>> = None;
static mut ESP_WINDOW: OnceCell<EspWindow> = OnceCell::new();

struct EspWindow {
    rects: Arc<Mutex<Vec<[f32; 2]>>>,
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

fn on_disable(esp: &mut Esp) {
    esp.sender.send(()).unwrap();
    unsafe {
        ESP_JOINHANDLE.take().unwrap().join().unwrap();
    }
}

fn on_enable() {
    let [width, height] = DIMENSIONS.get_or_init(|| unsafe {
        let hdc = ESP_HDC.unwrap_or_else(|| wglGetCurrentDC());
        let width = GetDeviceCaps(hdc, LOGPIXELSX);
        let height = GetDeviceCaps(hdc, LOGPIXELSY);
        [width, height]
    });
    unsafe {
        ESP_JOINHANDLE = Some(std::thread::spawn(move || {
            let mut event_loop: EventLoop<()> = EventLoop::new_any_thread();
            let window = {
                let size: LogicalSize<i32> = LogicalSize::new(*width, *height);
                let window = WindowBuilder::new()
                    .with_min_inner_size(size)
                    .with_decorations(false)
                    .with_resizable(false)
                    .with_always_on_top(true)
                    .with_maximized(true)
                    .with_transparent(true)
                    .with_resizable(false)
                    .build(&event_loop)
                    .unwrap();
                window.set_ignore_cursor_events(true).unwrap();
                window
            };

            let mut pixels = {
                let size = window.inner_size();
                let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
                let mut pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();
                pixels.set_clear_color(pixels::wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                });
                pixels
            };

            event_loop.run_return(move |event, _, control_flow| {
                let esp_window = ESP_WINDOW.get().unwrap().clone();
                if Arc::clone(&esp_window.receiver).as_ref().as_ref().unwrap().try_recv().is_ok() {  // magic
                    *control_flow = ControlFlow::Exit;
                }
                if let Event::RedrawRequested(_) = event {
                    let mut rects = esp_window.rects.lock().unwrap();
                    let frame = pixels.get_frame_mut();
                    for rect in rects.iter() {
                        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                            let x = i as f32 % *width as f32;
                            let y = i as f32 / *height as f32;
                            if (x >= rect[0] && x <= rect[0] + 10.0) || (y >= rect[1] && y <= rect[1] + 10.0) {
                                pixel.copy_from_slice(&[255, 255, 255, 255]);
                            }
                        }
                    }
                    rects.clear();

                    // gl stuff might not be needed
                    if ESP_HDC.is_none() {
                        ESP_HDC = Some(wglGetCurrentDC());
                    }
                    if ESP_CONTEXT.is_none() {
                        ESP_CONTEXT = Some(wglGetCurrentContext());
                    }
                    let hdc = *ESP_HDC.as_ref().unwrap();
                    let context = ESP_CONTEXT.as_mut().unwrap();
                    wglMakeCurrent(hdc, *context);

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
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
    on_disable_method = "on_disable(self)",
    on_enable_method = "on_enable()"
)]
pub struct Esp {
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
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            sender,
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}
