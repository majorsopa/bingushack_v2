use crate::crate_prelude::*;

fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);

    // debug
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    

    
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "Triggerbot", tick_method = "tick(_env, _mappings_manager)")]
pub struct Triggerbot {

}

impl MakeNewBingusModule for Triggerbot {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
        }
    }
}
