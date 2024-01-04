use crate::{
    chara::Character,
    player::{Player, Token},
};
use serde_derive::{Deserialize, Serialize};

/// All possible requests are defined here.
#[derive(Serialize, Deserialize)]
pub enum Req {
    /// Get current server status.
    ServerStatus,
    /// Synchronize all available data.
    Sync(Token),
    /// Receive out-game chat messages.
    RecvChat,
    /// Synchronize player list.
    GetPlayer,
    /// Synchronize character list.
    GetChara,
    /// Enroll in the game.
    Enroll(Character, Token),
    /// Send out-game chat message.
    SendChat(Token),
    /// Create, edit or delete player.
    EditPlayer(Player, Option<Token>),
    /// Create, edit or delete character.
    EditChara(Character, Token),
    /// Submit in-game action.
    Act(String, Token),
    /// Issue server command.
    Cmd(String, Token),
}

impl Req {
    pub fn route(&self) -> &'static str {
        match self {
            Req::ServerStatus => "GET /",
            Req::Sync(_) => "GET /sync",
            Req::RecvChat => "GET /chat",
            Req::GetPlayer => "GET /player",
            Req::GetChara => "GET /chara",
            Req::Enroll(_, _) => "POST /enroll",
            Req::SendChat(_) => "POST /chat",
            Req::EditPlayer(_, _) => "POST player",
            Req::EditChara(_, _) => "POST chara",
            Req::Act(_, _) => "POST /act",
            Req::Cmd(_, _) => "POST /cmd",
        }
    }
}

#[test]
fn see_json() {
    use crate::{
        chara::{Attr, Character},
        item::{Item, OtherItem},
        player::{Player, Token},
        util::Bottle,
    };
    use serde_json::to_string as json;
    let a = Attr::new("example_attribution", Some(Bottle::new(10)));
    let i = Item::Other(OtherItem::new(
        0,
        "example_id",
        "example_name",
        "This is description of an OtherItem.",
    ));
    let p = Player::new("example_player", "Player P Example", None);
    let t = Token::new("example_player", "example_password");
    let c = Character::new(
        &p.id,
        "example_character",
        "This is an example character",
        vec![a],
        vec![Some(i)],
    );
    println!("{}", json(&Req::ServerStatus).unwrap());
    println!("{}", json(&Req::Sync(t.clone())).unwrap());
    println!("{}", json(&Req::RecvChat).unwrap());
    println!("{}", json(&Req::GetPlayer).unwrap());
    println!("{}", json(&Req::GetChara).unwrap());
    println!("{}", json(&Req::Enroll(c.clone(), t.clone())).unwrap());
    println!("{}", json(&Req::SendChat(t.clone())).unwrap());
    println!("{}", json(&Req::EditPlayer(p, Some(t.clone()))).unwrap());
    println!("{}", json(&Req::EditChara(c.clone(), t.clone())).unwrap());
    println!(
        "{}",
        json(&Req::Act("Say hello world.".to_string(), t.clone())).unwrap()
    );
    println!(
        "{}",
        json(&Req::Cmd("help --help".to_string(), t.clone())).unwrap()
    )
}
