use crate::crate_prelude::*;

use super::bingus_module_trait::MakeNewBingusModule;

#[enum_dispatch(BingusModuleTrait)]
pub enum BingusModule {
    Autototem,
    TotemAssist,
    FakeGhost,
    //Esp,
}

pub fn populate_modules() -> Vec<BingusModule> {
    let mut modules = Vec::new();

    modules.push(Autototem::new().into());
    modules.push(TotemAssist::new().into());
    modules.push(FakeGhost::new().into());
    //modules.push(Esp::new().into());

    modules
}
