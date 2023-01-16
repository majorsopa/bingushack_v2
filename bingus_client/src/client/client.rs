use bingus_module::prelude::{BingusModule, populate_modules};

pub struct BingusClient {
    modules: Vec<BingusModule>,
}

impl BingusClient {
    pub fn new() -> Self {
        let new_self = Self {
            modules: populate_modules(),
        };

        new_self
    }
}
