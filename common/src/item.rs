use crate::{age::Age, dice::Dice, fight::DmgType, Feature, ID};
#[cfg(test)]
use crate::{Example, DEBUG_DESCR};
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

#[cfg(test)]
impl Example for Species {
    fn examples() -> Vec<Self> {
        vec![Self::Human, Self::NonHuman, Self::Else("cat".to_string())]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArmorLayer {
    pub material: ID,
    pub cover: HashSet<ID>,
    pub rate: u16,
    pub thickness: u16,
}

#[cfg(test)]
impl Example for ArmorLayer {
    fn examples() -> Vec<Self> {
        let mut cover = HashSet::new();
        cover.extend(ID::examples());
        vec![Self {
            material: ID::example(),
            cover,
            rate: 95,
            thickness: 3000,
        }]
    }
}

/// Instance of armor.
#[derive(Clone, Serialize, Deserialize)]
pub struct Armor {
    /// Damage
    pub def: Dice,
    /// Species able to wear this armor.
    pub species: Species,
    pub layer: Vec<ArmorLayer>,
    /// Whether resists penetration.
    pub resist_pntr: bool,
}

#[cfg(test)]
impl Example for Armor {
    fn examples() -> Vec<Self> {
        Species::examples()
            .into_iter()
            .map(|s| Self {
                def: "11d45+14".to_string(),
                species: s,
                layer: ArmorLayer::examples(),
                resist_pntr: true,
            })
            .collect()
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
    pub name: String,
    pub descr: String,
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

#[cfg(test)]
impl Example for ItemInst {
    fn examples() -> Vec<Self> {
        ItemSpec::examples()
            .into_iter()
            .map(|s| Self {
                name: "Example Item Instance".to_string(),
                descr: DEBUG_DESCR.to_string(),
                length: 114,
                volume: 514,
                weight: 514,
                price: 1919810,
                feature: HashSet::new(),
                ext_info: vec![],
                spec: Box::new(s),
            })
            .collect()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Item {
    Reg(ID),
    Inst(ItemInst),
}

#[cfg(test)]
impl Example for Item {
    fn examples() -> Vec<Self> {
        let mut v: Vec<_> = ItemInst::examples()
            .into_iter()
            .map(|i| Self::Inst(i))
            .collect();
        v.push(Self::Reg(ID::example()));
        v
    }
}

#[test]
fn see_json() {
    use serde_json::to_string as json;
    for i in Item::examples() {
        println!("{}", json(&i).unwrap());
    }
}
