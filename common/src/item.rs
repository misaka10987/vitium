use crate::{age::Age, dice::Dice, DEBUG_DESCR, ID, UID};
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type Price = HashMap<Age, u64>;

/// Instance of weapon.
#[derive(Serialize, Deserialize, Clone)]
pub struct Weapon {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: u64,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: String,
    /// Name dieplayed in game.
    pub name: String,
    /// Description displayed in game.
    pub descr: String,
    /// Age periods available.
    pub age: HashSet<Age>,
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
    fn uid(&self) -> u64 {
        self.uid
    }
    fn set_uid(&mut self, uid: u64) -> &mut Self {
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
            descr: DEBUG_DESCR.to_string(),
            age: HashSet::new(),
            atk: "11d45+14".to_string(),
            rng: 114514,
            pntr: true,
            per_turn: 11,
            charge: 45,
            load: 14,
            price: HashMap::new(),
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
    pub uid: u64,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: String,
    /// Name dieplayed in game.
    pub name: String,
    /// Description displayed in game.
    pub descr: String,
    /// Age periods available.
    pub age: HashSet<Age>,
    /// Damage
    pub def: Dice,
    /// Covered body parts.
    pub cover: HashSet<BodyPart>,
    /// Species able to wear this armor.
    pub species: Species,
    /// Whether resists penetration.
    pub rerist_pntr: bool,
    /// Price in different time periods.
    pub price: Price,
}

impl UID for Armor {
    fn uid(&self) -> u64 {
        self.uid
    }
    fn set_uid(&mut self, uid: u64) -> &mut Self {
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
            descr: DEBUG_DESCR.to_string(),
            age: HashSet::new(),
            def: "11d45+14".to_string(),
            cover: HashSet::new(),
            species: Species::Human,
            rerist_pntr: true,
            price: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OtherItem {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: u64,
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
    fn uid(&self) -> u64 {
        self.uid
    }
    fn set_uid(&mut self, uid: u64) -> &mut Self {
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
            descr: DEBUG_DESCR.to_string(),
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
    fn uid(&self) -> u64 {
        match self {
            Item::Weapon(i) => i.uid,
            Item::Armor(i) => i.uid,
            Item::Other(i) => i.uid,
        }
    }
    fn set_uid(&mut self, uid: u64) -> &mut Self {
        match self {
            Item::Weapon(i) => i.uid = uid,
            Item::Armor(i) => i.uid = uid,
            Item::Other(i) => i.uid = uid,
        }
        self
    }
}

pub type Inventory = Vec<Option<Item>>;
