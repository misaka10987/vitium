#[cfg(test)]
use crate::Example;
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

#[cfg(test)]
impl Example for ItemSpec {
    fn examples() -> Vec<Self> {
        vec![
            Self::Generic,
            Self::Container(vec![]),
            Self::Melee(Melee::example()),
            Self::Ranged(Ranged::example()),
            Self::Food(Food::example()),
            Self::Medicine(Medicine::example()),
        ]
    }
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

#[cfg(test)]
impl Example for Melee {
    fn examples() -> Vec<Self> {
        let mut atk = HashMap::new();
        atk.insert(DmgType::System, "11d45+14".to_string());
        let mut skill = HashSet::new();
        skill.extend(ID::examples());
        let mut m_art = HashSet::new();
        m_art.extend(ID::examples());
        vec![Self {
            atk,
            rng: 514,
            one_hand: true,
            skill,
            m_art,
        }]
    }
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

#[cfg(test)]
impl Example for Ranged {
    fn examples() -> Vec<Self> {
        let mut atk = HashMap::new();
        atk.insert(DmgType::System, "11d45+14".to_string());
        let mut charge = HashSet::new();
        charge.extend(ID::examples());
        vec![Self {
            atk,
            rng: 114.514,
            moa: 1.14514,
            speed: 114.514,
            charge,
            load: 114,
            one_shot: 2,
            per_turn: 2,
        }]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Food {}

#[cfg(test)]
impl Example for Food {
    fn examples() -> Vec<Self> {
        vec![Self {}]
    }
}

/// todo
#[derive(Clone, Serialize, Deserialize)]
pub struct Medicine {}

#[cfg(test)]
impl Example for Medicine {
    fn examples() -> Vec<Self> {
        vec![Self {}]
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
    pub cover: HashSet<ID>,
    /// Species able to wear this armor.
    pub species: Species,
    /// Whether resists penetration.
    pub resist_pntr: bool,
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
