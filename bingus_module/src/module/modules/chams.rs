use std::ffi::c_void;
use jvmti::agent::Agent;
use jvmti::environment::jvmti::JVMTIEnvironment;
use jvmti::native::JVMTIEnvPtr;
use jvmti::runtime::ClassFileLoadEvent;
use jvmti::native::JavaVMPtr;
use jvmti::capabilities::Capabilities;

use crate::crate_prelude::*;

const CLASS_TO_CHANGE: &'static str = "fip";

static mut HOOKED: bool = false;
static mut CHANGED: bool = false;

fn on_toggle(env: JNIEnv) {
    let vm: JavaVMPtr = env.get_java_vm().unwrap().get_java_vm_pointer() as *mut *const _;

    if unsafe { !HOOKED } {
        unsafe { HOOKED = true; }
        // "Agent_OnLoad" but not really
        let mut agent = Agent::new_with_capabilities(vm, Capabilities {
            can_retransform_classes: true,
            can_retransform_any_class: true,
            can_generate_all_class_hook_events: true,
            ..Default::default()
        });

        agent.on_class_file_load(Some(on_class_file_load));

        agent.update();
    }



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
            can_generate_all_class_hook_events: true,
            ..Default::default()
        }.to_native());
        let capa_ptr: *mut jvmtiCapabilities = std::ptr::null_mut() as *mut jvmtiCapabilities;
        (**env_ptr).GetCapabilities.unwrap()(env_ptr, capa_ptr);
        JVMTIEnvironment::new(env_ptr)
    };


    unsafe {
        (**jvmti_env.jvmti).RetransformClasses.unwrap()(
            jvmti_env.jvmti,
            1,
            {
                let jni_env = env.get_native_interface() as jvmti::native::JNIEnvPtr;
                let fip = std::ffi::CString::new(CLASS_TO_CHANGE).unwrap();
                let fip_ptr: *const std::ffi::c_char  = fip.as_ptr();
                &(**jni_env).FindClass.unwrap()(jni_env, fip_ptr)
            },
        );
    }
}

fn on_class_file_load(event: ClassFileLoadEvent) -> Option<Vec<u8>> {
    // todo: cache original class file

    if event.class_name == CLASS_TO_CHANGE {
        let bytes = match unsafe { CHANGED } {  // 202,254,186,190 == 0xCAFEBABE
            true => Vec::from(include_bytes!("fip_og.class").as_slice()),
            false => Vec::from(include_bytes!("fip_modded.class").as_slice()),
        };
        unsafe { CHANGED = !CHANGED; }
        Some(bytes)
    } else {
        None
    }
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(
    name = "Chams (buggy with armor)",
    on_enable_method = "on_toggle(_env)",
    on_disable_method = "on_toggle(_env)",
)]
pub struct Chams {}

impl MakeNewBingusModule for Chams {
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
