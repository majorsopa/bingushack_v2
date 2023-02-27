use std::ffi::c_void;

use jni::JavaVM;

#[cfg(target_os = "windows")]

mod mappings_manager;

pub use mappings_manager::*;


pub unsafe fn get_javavm() -> JavaVM {
    use jni::sys::JNI_GetCreatedJavaVMs;

    let jvm_ptr = Vec::with_capacity(1).as_mut_ptr();
    JNI_GetCreatedJavaVMs(jvm_ptr, 1, std::ptr::null_mut());

    JavaVM::from_raw(*jvm_ptr).unwrap()
}

// return value is a success code
pub unsafe fn get_jvmti(jvm: JavaVM, jvmti_ptr_buffer: *mut *mut c_void) -> i32 {
    use jvm_rs::jvmti::JVMTI_VERSION_1_0;
    use jni::sys::JNIInvokeInterface_;

    let jvm: *mut JNIInvokeInterface_ = std::mem::transmute(jvm.get_java_vm_pointer());

    (*jvm).GetEnv.unwrap()(std::mem::transmute(jvm), jvmti_ptr_buffer, JVMTI_VERSION_1_0 as i32)
}
