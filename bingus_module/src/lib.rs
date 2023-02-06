#![feature(get_mut_unchecked)]

use std::sync::{Mutex, Arc, atomic::AtomicPtr};

use once_cell::sync::OnceCell;

mod module;
mod crate_prelude;
mod utilities;

pub mod prelude;

pub static GHOST_MODE: OnceCell<Arc<Mutex<(bool, AtomicPtr<winapi::shared::windef::HWND>)>>> = OnceCell::new();
