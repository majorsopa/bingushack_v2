mod client;

use std::{ptr::null_mut, ffi::CString};

pub use client::*;
use winapi::um::winuser::{MessageBoxA, MB_OK};

pub fn message_box(text: &str) {
    let caption = CString::new("bingushack").unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        MessageBoxA(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}
