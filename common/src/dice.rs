use serde_derive::{Deserialize, Serialize};

/// Standard dice expression, eg `1d2+3`.
pub type Dice = String;
/// Defines all possible exceptions rolling dice.
#[derive(Serialize, Deserialize, Clone)]
pub enum DError {
    Parse(String),
    Roll(String),
}
/// Result of a dice expression.
pub type DResult = Result<i32, DError>;
