use crate::crate_prelude::*;

#[enum_dispatch]
pub trait BingusSettingTrait<T> {
    fn get_value(&self) -> T;
    fn get_value_mut(&mut self) -> &mut T;
}
