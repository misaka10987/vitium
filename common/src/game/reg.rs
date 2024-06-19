use std::sync::Arc;

use crate::t_recs::reg::RegTab;

use super::{ItemReg, Mat, Terra};

pub struct GameReg {
    pub item: Arc<ItemReg>,
    pub mat: Arc<RegTab<Mat>>,
    pub terra: Arc<RegTab<Terra>>,
}
