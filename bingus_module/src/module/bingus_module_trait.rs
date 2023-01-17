use crate::crate_prelude::*;


#[enum_dispatch]
pub trait BingusModuleTrait {
    fn get_name(&self) -> &'static str;

    fn tick(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);
    fn on_enable(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);
    fn on_disable(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);
    fn on_load(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);
    fn on_unload(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);

    fn toggle(&mut self, _env: JNIEnv, _mappings_manager: &MappingsManager);

    fn get_enabled(&self) -> (BingusSetting, &'static str);
    fn get_enabled_mut(&mut self) -> &mut (BingusSetting, &'static str);

    fn get_settings(&self) -> Vec<(BingusSetting, &'static str)>;
    fn get_settings_mut(&mut self) -> Vec<&mut (BingusSetting, &'static str)>;
}

pub trait MakeNewBingusModule {
    fn new() -> Self;
}
