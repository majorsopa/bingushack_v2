use crate::crate_prelude::*;

#[enum_dispatch]
pub trait BingusSettingTrait<T> {
    fn get_value(&mut self) -> &mut T;
}
