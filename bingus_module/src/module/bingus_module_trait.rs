use crate::crate_prelude::*;


#[enum_dispatch]
pub trait BingusModuleTrait {
    fn init(&mut self, jni_env: JNIEnv<'static>, mappings_manager: &mut Arc<MappingsManager<'static>>, hwnd: &mut Arc<winapi::shared::windef::HWND>);
    fn get_name(&self) -> &'static str;

    fn tick(&mut self);
    fn render(&mut self);

    fn on_enable(&mut self);
    fn on_disable(&mut self);
    fn on_load(&mut self);        // does nothing atm
    fn on_unload(&mut self);      // does nothing atm

    fn toggle(&mut self);

    fn get_enabled(&mut self) -> (&mut BingusSetting, &'static str, Option<[f32; 2]>);  // odd that these needs to be mutable but yes for now they do
    fn get_keybind(&mut self) -> (&mut BingusSetting, &'static str, Option<[f32; 2]>);
    fn get_settings(&mut self) -> Vec<(&mut BingusSetting, &'static str, Option<[f32; 2]>)>;
}

pub trait MakeNewBingusModule {
    fn new() -> Self;
}
