mod client;

use std::sync::{Arc, Mutex};

use bingus_module::prelude::BingusModule;
pub use client::*;
use once_cell::sync::OnceCell;

pub static MODULES: OnceCell<Arc<Mutex<Vec<BingusModule>>>> = OnceCell::new();
