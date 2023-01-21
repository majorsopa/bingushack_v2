use crate::crate_prelude::*;

pub fn is_alive<'a>(env: JNIEnv<'a>, target: &'a ClassMapping) -> bool {
    call_method_or_get_field!(env, target, "isAlive", false, &[]).unwrap().z().unwrap()
}