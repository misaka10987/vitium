use crate::{age::Age, dice::Dice, fight::DmgType, Feature, DEBUG_DESCR, ID};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type Price = HashMap<Age, u64>;

#[derive(Clone, Serialize, Deserialize)]
pub enum ItemSpec {
    Generic,
    Container(Vec<Item>),
    Melee(Melee),
    Ranged(Ranged),
    Food(Food),
    Medicine(Medicine),
    Armor(Armor),
    Other(OtherItem),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Melee {
    pub atk: HashMap<DmgType, Dice>,
    /// In milimetres.
    pub rng: u16,
    pub one_hand: bool,
    pub skill: HashSet<ID>,
    pub m_art: HashSet<ID>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Ranged {
    pub atk: HashMap<DmgType, Dice>,
    /// In metres.
    pub rng: f32,
    pub moa: f32,
    pub speed: f32,
    pub charge: HashSet<ID>,
    pub load: u16,
    pub one_shot: u8,
    pub per_turn: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Food {}

/// todo
#[derive(Clone, Serialize, Deserialize)]
pub struct Medicine {}

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
    pub resist_pntr: bool,
}

impl Armor {
    pub fn new() -> Self {
        Self {
            def: "11d45+14".to_string(),
            cover: HashSet::new(),
            species: Species::Human,
            resist_pntr: true,
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
pub struct ItemInst {
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
    pub feature: HashSet<Feature>,
    pub ext_info: Vec<String>,
    /// Detailed class, like weapon and armor.
    pub spec: Box<ItemSpec>,
}

impl ItemInst {
    pub fn example() -> Self {
        Self {
            name: Some("Example Item".to_string()),
            descr: Some(DEBUG_DESCR.to_string()),
            length: 114,
            volume: 514,
            weight: 514,
            price: 1919810,
            feature: HashSet::new(),
            ext_info: vec![],
            spec: Box::new(ItemSpec::Generic),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Item {
    Reg(ID),
    Inst(ItemInst),
}
