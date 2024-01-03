use vitium_common::dice::{DError, DResult, Dice};

/// Inplemented for dice.
pub trait Roll {
    fn roll(&self) -> DResult;
}
impl Roll for Dice {
    fn roll(&self) -> DResult {
        match self.parse::<ndm::Dice>() {
            Ok(dice) => Ok(dice.total()),
            Err(derr) => Err(DError::Parse(derr.to_string())),
        }
    }
}
