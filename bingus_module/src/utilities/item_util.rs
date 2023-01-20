use crate::crate_prelude::*;

pub fn get_raw_id_of_item<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, item_name: &str) -> i32 {
    let item_class = mappings_manager.get("Item").unwrap();

    let item_to_get = mappings_manager.get("Items").unwrap();
    apply_object!(
        item_to_get,
        call_method_or_get_field!(env, item_to_get, item_name, true).unwrap().l().unwrap()
    );

    call_method_or_get_field!(
        env,
        item_class,
        "getRawId",
        true,
        &[JValue::from(item_to_get.get_object().unwrap())]
    )
    .unwrap()
    .i()
    .unwrap()
}

pub fn get_raw_id_of_item_object<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, item_object: &'a ClassMapping<'a>) -> i32 {
    let item_class = mappings_manager.get("Item").unwrap();

    call_method_or_get_field!(
        env,
        item_class,
        "getRawId",
        true,
        &[JValue::from(item_object.get_object().unwrap())]
    )
    .unwrap()
    .i()
    .unwrap()
}

pub fn item_from_item_stack<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, item_stack: &'a ClassMapping<'a>) -> &'a ClassMapping<'a> {
    let item = mappings_manager.get("Item").unwrap();
    apply_object!(
        item,
        call_method_or_get_field!(env, item_stack, "getItem", false, &[]).unwrap().l().unwrap()
    );
    item
}