use tracing::warn;
use vitium_common::{
    game::act::walk::{Walk, WalkRes},
    player::NoPlayerError,
};

use crate::game::Proc;

use super::Slave;

impl Proc<Walk> for Slave {
    fn proc(
        &mut self,
        pc: String,
        act: Walk,
    ) -> Result<<Walk as vitium_common::game::prelude::Act>::Res, Box<dyn std::error::Error>> {
        warn!("walk has not been implemented yet");
        if let Some(pc) = self.scena.pc.get_mut(&pc) {
            let (x1, y1) = pc.pos.coord;
            let (x2, y2) = act.dest;
            let (dx, dy) = (x1 - x2, y1 - y2);
            let d = (dx * dx + dy * dy).sqrt();
            let (x, y) = (x2 as i16, y2 as i16);
            if false
                || {
                    let _ = self.scena.map.block(x, y);
                    false
                }
                || act.speed > 114514.0
                || d > 114514.0
            {
                return Ok(WalkRes::Forbidden);
            }
            pc.pos.coord = (x2, y2);
            Ok(WalkRes::Success)
        } else {
            Err(NoPlayerError(pc).into())
        }
    }
}
