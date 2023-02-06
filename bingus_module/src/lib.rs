#![feature(get_mut_unchecked)]

use std::sync::{Mutex, Arc};

use once_cell::sync::OnceCell;  // me when the when the the

mod module;
mod crate_prelude;
mod utilities;

pub mod prelude;

pub static GHOST_MODE: OnceCell<Arc<Mutex<bool>>> = OnceCell::new();
