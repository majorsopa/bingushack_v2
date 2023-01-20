use crate::crate_prelude::*;

fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    let target = mappings_manager.get("Optional").unwrap();
    apply_object!(
        target,
        call_method_or_get_field!(
            env,
            mappings_manager.get("DebugRenderer").unwrap(),
            "getTargetedEntity",
            true,
            &[
                JValue::from(player.get_object().unwrap()),
                JValue::from(3),
            ]
        ).unwrap().l().unwrap()
    );
    if !call_method_or_get_field!(env, target, "isPresent", false, &[]).unwrap().z().unwrap() {
        return;
    }
    let target = {
        let entity = mappings_manager.get("Entity").unwrap();
        apply_object!(
            entity,
            call_method_or_get_field!(
                env,
                target,
                "get",
                false,
                &[]
            ).unwrap().l().unwrap()
        );
        entity
    };
    if {
        !call_method_or_get_field!(env, target, "isAlive", false, &[]).unwrap().z().unwrap()
        || call_method_or_get_field!(env, player, "isUsingItem", false, &[]).unwrap().z().unwrap()
        || (call_method_or_get_field!(env, player, "getAttackCooldownProgress", false, &[
            call_method_or_get_field!(env, minecraft_client, "getTickDelta", false, &[]).unwrap(),
        ]).unwrap().f().unwrap() != 1.0 )
    } {
        return;
    }

    let interaction_manager = get_interaction_manager(env, mappings_manager, minecraft_client);
    call_method_or_get_field!(
        env,
        interaction_manager,
        "attackEntity",
        false,
        &[
            JValue::from(player.get_object().unwrap()),
            JValue::from(target.get_object().unwrap()),
        ]
    ).unwrap();
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
