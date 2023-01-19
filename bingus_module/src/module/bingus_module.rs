use crate::crate_prelude::*;

use super::bingus_module_trait::MakeNewBingusModule;

#[enum_dispatch(BingusModuleTrait)]
pub enum BingusModule {
    ChatSender,
    Esp,
}

pub fn populate_modules() -> Vec<BingusModule> {
    let mut modules = Vec::new();

    modules.push(ChatSender::new().into());
    modules.push(Esp::new().into());

    modules
}
