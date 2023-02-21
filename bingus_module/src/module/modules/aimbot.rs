use crate::crate_prelude::*;

fn tick(aimbot: &mut Aimbot, env: JNIEnv, mappings_manager: &MappingsManager) {
    // get the entity position

    // get pitch

    // get yaw
}

// returns the delta that should be achieved by the next rotation
// rn it's only linear
fn next_smooth_rotation(pitch_and_yaw: [f32; 2], goal: [f32; 2], interval: [f32; 2]) -> [f32; 2] {
    let ret: [f32; 2] = [0.0, 0.0];
    todo!()
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "Aimbot", tick_method = "tick(self, _env, _mappings_manager)", settings_list_fields = "[range, horizontal_radius, vertical_radius, player_only]")]
pub struct Aimbot {
    range: (BingusSetting, &'static str, Option<[f32; 2]>),
    horizontal_radius: (BingusSetting, &'static str, Option<[f32; 2]>),
    vertical_radius: (BingusSetting, &'static str, Option<[f32; 2]>),
    player_only: (BingusSetting, &'static str, Option<[f32; 2]>),
}

impl MakeNewBingusModule for Aimbot {
    fn new() -> Self {
        Self {
            range: (BingusSetting::FloatSetting(3.5.into()), "range", Some([0.1, 6.0])),
            horizontal_radius: (BingusSetting::FloatSetting(20.0.into()), "horizontal radius", Some([1.0, 90.0])),
            vertical_radius: (BingusSetting::FloatSetting(15.0.into()), "vertical radius", Some([1.0, 90.0])),
            player_only: (BingusSetting::BoolSetting(true.into()), "player only", None),

            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}
