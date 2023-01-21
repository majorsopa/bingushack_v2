use crate::crate_prelude::*;

pub fn get_world_checked<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> Option<&'a ClassMapping<'a>> {
    let world = mappings_manager.get("ClientWorld").unwrap();
    apply_object!(
        world,
        {
            let check_if_null = call_method_or_get_field!(env, minecraft_client, "world", false).unwrap().l().unwrap();
            if env.is_same_object(check_if_null, JObject::null()).unwrap() {
                return None;
            } else {
                check_if_null
            }
        }
    );
    Some(world)
}