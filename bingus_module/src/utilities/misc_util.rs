use crate::crate_prelude::*;

pub fn is_instance_of<'a>(env: JNIEnv<'a>, object: &'a ClassMapping<'a>, class: &'a ClassMapping<'a>) -> bool {
    env.is_instance_of(object.get_object().unwrap(), class.get_class()).unwrap()
}

pub fn get_simple_option<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, simple_option: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let value = mappings_manager.get("Object").unwrap();
    apply_object!(
        value,
        call_method_or_get_field!(
            env,
            simple_option,
            "value",
            false
        ).unwrap().l().unwrap()
    );
    value
}

pub fn set_simple_option<'a>(env: JNIEnv<'a>, simple_option: &'a ClassMapping, value: &'a ClassMapping<'a>) {
    let sig_holder = simple_option.get_field("value", false).unwrap();
    let name = sig_holder.get_name();
    let ty = sig_holder.get_sig();
    env.set_field(
        simple_option.get_object().unwrap(),
        name,
        ty,
        JValue::from(value.get_object().unwrap())
    ).unwrap();
}