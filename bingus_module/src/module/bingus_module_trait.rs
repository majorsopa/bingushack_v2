use crate::crate_prelude::*;


#[enum_dispatch]
pub trait BingusModuleTrait {
    fn get_name(&self) -> &'static str;

    fn tick(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);
    fn render(&mut self);

    fn on_enable(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);      // do nothing atm
    fn on_disable(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);     // do nothing atm
    fn on_load(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);        // do nothing atm
    fn on_unload(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);      // do nothing atm

    fn toggle(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);

    fn get_enabled(&mut self) -> (&mut BingusSetting, &'static str, Option<[f32; 2]>);  // odd that these needs to be mutable but yes for now they do
    fn get_keybind(&mut self) -> (&mut BingusSetting, &'static str, Option<[f32; 2]>);
    fn get_settings(&mut self) -> Vec<(&mut BingusSetting, &'static str, Option<[f32; 2]>)>;
}

pub trait MakeNewBingusModule {
    fn new() -> Self;
}
