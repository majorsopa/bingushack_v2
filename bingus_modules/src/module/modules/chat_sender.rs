use crate::crate_prelude::*;


fn tick() {
    println!("tick called");
}


#[derive(BingusModuleTrait)]
#[bingus_module(name = "ChatSender", tick_method = "tick()")]
pub struct ChatSender;
