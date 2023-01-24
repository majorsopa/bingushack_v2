mod client;

use std::{ptr::null_mut, ffi::CString, sync::{Arc, Mutex, atomic::AtomicPtr}};

use bingus_module::prelude::BingusModule;
pub use client::*;
use once_cell::sync::OnceCell;
use winapi::um::winuser::{MessageBoxA, MB_OK};

pub static MODULES: OnceCell<Arc<Mutex<Vec<BingusModule>>>> = OnceCell::new();


pub fn message_box(text: &str) {
    let caption = CString::new("bingushack").unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        MessageBoxA(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}
