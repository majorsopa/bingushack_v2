use crate::crate_prelude::*;

pub fn get_minecraft_client<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> &'a ClassMapping<'a> {
    let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
    apply_object!(
        minecraft_client,
        call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
    );
    minecraft_client
}

pub fn get_interaction_manager<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let interaction_manager = mappings_manager.get("InteractionManager").unwrap();
    apply_object!(
        interaction_manager,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "interactionManager",
            false
        ).unwrap().l().unwrap()
    );
    interaction_manager
}