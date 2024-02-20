use crate::{age::Age, dice::Dice, DEBUG_DESCR, ID, UID};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type Price = HashMap<Age, u64>;

#[derive(Clone, Serialize, Deserialize)]
pub enum ItemSpec {
    Weapon(Weapon),
    Armor(Armor),
    Other(OtherItem),
}

/// Instance of weapon.
#[derive(Clone, Serialize, Deserialize)]
pub struct Weapon {
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

impl Weapon {
    pub fn new() -> Self {
        Self {
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

/// Defines species for deciding if an armor is able to wear.
#[derive(Clone, Serialize, Deserialize)]
pub enum Species {
    /// Human-liked species.
    Human,
    /// Non human-liked species.
    NonHuman,
    /// Let host decide if able to wear.
    Else(String),
}

/// Instance of armor.
#[derive(Clone, Serialize, Deserialize)]
pub struct Armor {
    /// Damage
    pub def: Dice,
    /// Covered body parts.
    pub cover: HashSet<String>,
    /// Species able to wear this armor.
    pub species: Species,
    /// Whether resists penetration.
    pub rerist_pntr: bool,
    /// Price in different time periods.
    pub price: Price,
}

impl Armor {
    pub fn new() -> Self {
        Self {
            def: "11d45+14".to_string(),
            cover: HashSet::new(),
            species: Species::Human,
            rerist_pntr: true,
            price: HashMap::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OtherItem {}

impl OtherItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    /// The UID, set to 0 for not generated yet.
    pub uid: u64,
    pub id: Option<String>,
    pub name: Option<String>,
    pub descr: Option<String>,
    /// In milimetres.
    pub length: u16,
    /// In mililitres.
    pub volume: u16,
    /// In grams.
    pub weight: u16,
    /// In the smallest currency unit, like 1 USD cent.
    pub price: u32,
    /// Detailed class, like weapon and armor.
    pub spec: ItemSpec,
}

impl Item {
    pub fn new() -> Self {
        Self {
            uid: 0,
            id: None,
            name: Some("Example Item".to_string()),
            descr: Some(DEBUG_DESCR.to_string()),
            length: 114,
            volume: 514,
            weight: 514,
            price: 1919810,
            spec: ItemSpec::Other(OtherItem::new()),
        }
    }
}

impl ID for Item {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

impl UID for Item {
    fn uid(&self) -> u64 {
        self.uid
    }
    fn set_uid(&mut self, uid: u64) -> &mut Self {
        self.uid = uid;
        self
    }
}
