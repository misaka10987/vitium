use std::error::Error;

use vitium_common::game::Act;

pub trait Proc<T: Act> {
    fn proc(&mut self, pc: String, act: T) -> Result<T::Res, Box<dyn Error>>;
}
