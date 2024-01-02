use crate::{
    character::Character,
    player::{Player, Token},
};
use serde_derive::{Deserialize, Serialize};
/// All possible requests are defined here.
#[derive(Serialize, Deserialize)]
pub enum Req {
    /// Get current status
    Status,
    /// Enroll in a game
    Enroll(Character, Token),
    /// Synchronize all available data.
    Sync(Token),
    /// Ask to create new player
    NewPlayer(Player),
    /// Ask to create new character
    NewCharacter(Token),
    /// Submit action
    Act(String, Token),
    /// Out-game chat
    Chat(String, Token),
}
