use std::ffi::c_void;
use std::{time::SystemTime, ops::Deref};
use jvmti::agent::Agent;
use jvmti::environment::jvmti::JVMTIEnvironment;
use jvmti::native::JVMTIEnvPtr;
use jvmti::runtime::ClassFileLoadEvent;
use jvmti::native::JavaVMPtr;
use jvmti::capabilities::Capabilities;

use crate::crate_prelude::*;

fn on_enable(env: JNIEnv, mappings_manager: &MappingsManager) {
    let vm: JavaVMPtr = env.get_java_vm().unwrap().get_java_vm_pointer() as *mut *const _;

    // "Agent_OnLoad" but not really
    let mut agent = Agent::new(vm);

    let jvmti_env = unsafe {
        use jvmti::native::jvmti_native::jvmtiCapabilities;
        use jvmti::native::jvmti_native::JVMTI_VERSION;
    
        let mut void_ptr: *mut c_void = std::ptr::null_mut() as *mut c_void;
        let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
        (**vm).GetEnv.unwrap()(vm, penv_ptr, JVMTI_VERSION);
        let env_ptr: JVMTIEnvPtr = *penv_ptr as JVMTIEnvPtr;
        (**env_ptr).AddCapabilities.unwrap()(env_ptr, &Capabilities {
            can_retransform_classes: true,
            can_retransform_any_class: true,
            ..Default::default()
        }.to_native());
        let capa_ptr: *mut jvmtiCapabilities = std::ptr::null_mut() as *mut jvmtiCapabilities;
        (**env_ptr).GetCapabilities.unwrap()(env_ptr, capa_ptr);
        JVMTIEnvironment::new(env_ptr)
    };

    agent.on_class_file_load(Some(on_class_file_load));

    agent.update();


    unsafe {
    
        (**jvmti_env.jvmti).RetransformClasses.unwrap()(
            jvmti_env.jvmti,
            1,
            {
                let jni_env = env.get_native_interface() as jvmti::native::JNIEnvPtr;
                let eoc = std::ffi::CString::new("eoc").unwrap();
                let eoc_ptr: *const std::ffi::c_char  = eoc.as_ptr();
                &(**jni_env).FindClass.unwrap()(jni_env, eoc_ptr)
            },
        );
    }
}

fn on_class_file_load(event: ClassFileLoadEvent) -> Option<Vec<u8>> {
    use std::io::Read;

    // todo: cache original class file

    //println!("class file loaded: {}", event.class_name);

    if event.class_name == "eoc" {  // Minecraft
        println!("eoc class file loaded");
        panic!();
        // return `C:\Users\majorsopa\Desktop\1.19.3\eoc.class` as a vector of bytes
        //let mut file = std::fs::File::open("C:\\Users\\majorsopa\\Desktop\\1.19.3\\eoc.class").unwrap();
        //let mut bytes = Vec::new();
        //file.read_to_end(&mut bytes).unwrap();
        //Some(bytes)
    } else {
        //None
    }
    None
}

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
