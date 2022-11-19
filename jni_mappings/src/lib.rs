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
