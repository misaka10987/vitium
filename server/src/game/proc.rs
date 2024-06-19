use std::error::Error;

use tokio::sync::oneshot;
use vitium_common::game::Act;

pub trait Proc<T: Act> {
    fn proc(
        &mut self,
        pc: String,
        act: T,
    ) -> impl std::future::Future<Output = oneshot::Receiver<Result<T::Res, Box<dyn Error>>>>;
}
