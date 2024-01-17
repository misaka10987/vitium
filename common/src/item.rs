use crate::{age::Age, dice::Dice, DEBUG_MSG, ID, UID};
use serde_derive::{Deserialize, Serialize};
pub type Price = Vec<(Age, u64)>;

/// Instance of weapon.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Weapon {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: i128,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: String,
    /// Name dieplayed in game.
    pub name: String,
    /// Description displayed in game.
    pub descr: String,
    /// Age periods available.
    pub age: Vec<Age>,
    /// Damage expression using dice, eg `1d4+1`.
    pub atk: Dice,
    /// In milimetres, `0` for melee weapons.
    pub rng: u32,
    /// Whether to apply penetration.
    pub pntr: bool,
    /// Number of attacks able to inflict in a turn.
    pub per_turn: u8,
    /// Charges remaining.
    pub charge: u8,
    /// Charges used per attack.
    pub load: u8,
    /// Price in different time periods.
    pub price: Price,
}

impl UID for Weapon {
    fn uid(&self) -> i128 {
        self.uid
    }
    fn set_uid(&mut self, uid: i128) -> &mut Self {
        self.uid = uid;
        self
    }
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            uid: 0,
            id: "debug-weapon".to_string(),
            name: "Debug Weapon".to_string(),
            descr: DEBUG_MSG.to_string(),
            age: vec![],
            atk: "11d45+14".to_string(),
            rng: 114514,
            pntr: true,
            per_turn: 11,
            charge: 45,
            load: 14,
            price: vec![],
        }
    }
}

pub type BodyPart = String;

/// Defines species for deciding if an armor is able to wear.
#[derive(Serialize, Deserialize, Clone)]
pub enum Species {
    /// Human-liked species.
    Human,
    /// Non human-liked species.
    NonHuman,
    /// Let host decide if able to wear.
    Else(String),
}

/// Instance of armor.
#[derive(Serialize, Deserialize, Clone)]
pub struct Armor {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: i128,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: String,
    /// Name dieplayed in game.
    pub name: String,
    /// Description displayed in game.
    pub descr: String,
    /// Age periods available.
    pub age: Vec<Age>,
    /// Damage
    pub def: Dice,
    /// Covered body parts.
    pub cover: Vec<BodyPart>,
    /// Species able to wear this armor.
    pub species: Species,
    /// Whether resists penetration.
    pub rerist_pntr: bool,
    /// Price in different time periods.
    pub price: Price,
}

impl UID for Armor {
    fn uid(&self) -> i128 {
        self.uid
    }
    fn set_uid(&mut self, uid: i128) -> &mut Self {
        self.uid = uid;
        self
    }
}

impl Armor {
    pub fn new() -> Self {
        Self {
            uid: 0,
            id: "debug-armor".to_string(),
            name: "Debug Armor".to_string(),
            descr: DEBUG_MSG.to_string(),
            age: vec![],
            def: "11d45+14".to_string(),
            cover: vec![],
            species: Species::Human,
            rerist_pntr: true,
            price: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OtherItem {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: i128,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: String,
    /// Name displayed in game.
    pub name: String,
    /// Description displayed in game.
    pub descr: String,
}

impl UID for OtherItem {
    fn uid(&self) -> i128 {
        self.uid
    }
    fn set_uid(&mut self, uid: i128) -> &mut Self {
        self.uid = uid;
        self
    }
}

impl OtherItem {
    pub fn new() -> Self {
        Self {
            uid: 0,
            id: "debug-otheritem".to_string(),
            name: "Debug Other Item".to_string(),
            descr: DEBUG_MSG.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Other(OtherItem),
}

impl ID for Item {
    fn id(&self) -> String {
        match self {
            Item::Weapon(i) => i.id.clone(),
            Item::Armor(i) => i.id.clone(),
            Item::Other(i) => i.id.clone(),
        }
    }
}

impl UID for Item {
    fn uid(&self) -> i128 {
        match self {
            Item::Weapon(i) => i.uid,
            Item::Armor(i) => i.uid,
            Item::Other(i) => i.uid,
        }
    }
    fn set_uid(&mut self, uid: i128) -> &mut Self {
        match self {
            Item::Weapon(i) => i.uid = uid,
            Item::Armor(i) => i.uid = uid,
            Item::Other(i) => i.uid = uid,
        }
        self
    }
}

pub type Inventory = Vec<Option<Item>>;

// #[test]
// fn see_json() {
//     use serde_json::to_string as json;
//     let id = "example_id".to_string();
//     let a = vec!["1920s".to_string()];
//     let c = vec!["torso".to_string(), "head".to_string()];
//     let p = vec![("1920s".to_string(), 1919810)];
//     let i1 = Item::Weapon(Weapon::new(
//         0,
//         &id,
//         "example_name",
//         "This is an example weapon.",
//         a.clone(),
//         "11d45+14",
//         0,
//         true,
//         2,
//         5,
//         1,
//         p.clone(),
//     ));
//     let i2 = Item::Armor(Armor::new(
//         0,
//         &id,
//         "example_name",
//         "This is an example armor.",
//         a,
//         "11d45+14",
//         c,
//         Species::Human,
//         true,
//         p,
//     ));
//     let i3 = Item::Other(OtherItem::new(
//         0,
//         "example_id",
//         "example_name",
//         "This is description of an OtherItem.",
//     ));
//     println!("{}", json(&i1).unwrap());
//     println!("{}", json(&i2).unwrap());
//     println!("{}", json(&i3).unwrap());
//     let i: Inventory = vec![Some(i1), None];
//     println!("{}", json(&i).unwrap());
// }
