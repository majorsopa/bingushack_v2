use crate::crate_prelude::*;

#[enum_dispatch(BingusModuleTrait)]
pub enum BingusModule {
    ChatSender(ChatSender),
}