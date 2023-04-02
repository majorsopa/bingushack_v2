use std::{time::SystemTime, ops::Deref};

use jvmti::{native::{JavaVMPtr, JVMTIEnvPtr}, agent::Agent, runtime::{ClassFileLoadEvent, MethodInvocationEvent}, context::static_context, environment::jvmti::JVMTIEnvironment};

use crate::crate_prelude::*;

fn on_enable(env: JNIEnv, mappings_manager: &MappingsManager) {
    let vm = env.get_java_vm().unwrap().get_java_vm_pointer() as *mut *const _;

    // "Agent_OnLoad" but not really
    let mut agent = Agent::new(vm);

    //let jvmti_env = unsafe {
    //    use std::ffi::c_void;
    //    use jvmti::native::jvmti_native::JVMTI_VERSION;
    //
    //    let mut void_ptr: *mut c_void = std::ptr::null_mut() as *mut c_void;
    //    let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
    //    (**vm).GetEnv.unwrap()(vm, penv_ptr, JVMTI_VERSION);
    //    let env_ptr: JVMTIEnvPtr = *penv_ptr as JVMTIEnvPtr;
    //    JVMTIEnvironment::new(env_ptr)
    //};

    //agent.on_class_file_load(Some(on_class_file_load));

    agent.on_method_entry(Some(on_method_entry));

    agent.update();

    //unsafe {
    //    use jvmti::native::jvmti_native::Struct__jobject;
    //
    //    let r = (**jvmti_env.jvmti).RetransformClasses.unwrap()(
    //        jvmti_env.jvmti,
    //        1,
    //        std::mem::transmute(&mut Struct__jobject { _hacky_hack_workaround: *env.find_class("ejf").unwrap().deref().deref() as u32 })
    //    );
    //
    //    println!("RetransformClasses: {}", r);
    //}

}

fn on_method_entry(event: MethodInvocationEvent) {
    if event.class_sig.to_string() == "eoc" {
        println!("death screen!");
    }
}

/*
fn on_class_file_load(event: ClassFileLoadEvent) -> Option<Vec<u8>> {
    use std::io::Read;

    // todo: cache original class file

    //println!("class file loaded: {}", event.class_name);

    if event.class_name == "ejf" {  // Minecraft
        println!("ejf class file loaded");
        // return `C:\Users\majorsopa\Desktop\1.19.3\ejf.class` as a vector of bytes
        //let mut file = std::fs::File::open("C:\\Users\\majorsopa\\Desktop\\1.19.3\\ejf.class").unwrap();
        //let mut bytes = Vec::new();
        //file.read_to_end(&mut bytes).unwrap();
        //Some(bytes)
    } else {
        //None
    }
    None
}
*/

// todo: reset class file on disable

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(
    name = "FakeGhost (lawl)",
    on_enable_method = "on_enable(_env, _mappings_manager)"
)]
pub struct FakeGhost {}

impl MakeNewBingusModule for FakeGhost {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}
