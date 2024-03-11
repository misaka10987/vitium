use crate::{dice::Dice, fight::DmgType, Feature, ID};
#[cfg(test)]
use crate::{Example, DEBUG_DESCR};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Serialize, Deserialize)]
pub enum ItemSpec {
    Generic,
    Container(Box<Container>),
    Melee(Box<Melee>),
    Ranged(Box<Ranged>),
    Food(Box<Food>),
    Armor(Box<Armor>),
    Other(Box<OtherItem>),
}

#[cfg(test)]
impl Example for ItemSpec {
    fn examples() -> Vec<Self> {
        vec![
            Self::Generic,
            Self::Container(Box::new(Container::new())),
            Self::Melee(Box::new(Melee::example())),
            Self::Ranged(Box::new(Ranged::example())),
            Self::Food(Box::new(Food::example())),
        ]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Container {
    /// Time to store an item.
    pub time_cost: i32,
    /// In milimetres.
    pub length: i32,
    /// In mililitres.
    pub volume: i32,
    /// In grams.
    pub weight: i32,
    /// If the container is waterproof.
    pub waterproof: bool,
    pub inside: Vec<Item>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            time_cost: 114514,
            length: 114514,
            volume: 114514,
            weight: 114514,
            inside: vec![],
            waterproof: true,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Melee {
    pub atk: HashMap<DmgType, Dice>,
    /// In milimetres.
    pub rng: i32,
    pub one_hand: bool,
    pub skill: HashSet<ID>,
    pub mart: HashSet<ID>,
}

#[cfg(test)]
impl Example for Melee {
    fn examples() -> Vec<Self> {
        let mut atk = HashMap::new();
        atk.insert(DmgType::System, "11d45+14".to_string());
        let mut skill = HashSet::new();
        skill.extend(ID::examples());
        let mut mart = HashSet::new();
        mart.extend(ID::examples());
        vec![Self {
            atk,
            rng: 514,
            one_hand: true,
            skill,
            mart,
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
    pub load: i16,
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
pub struct Food {
    pub taste: i8,
    pub energy: i32,
    pub purified: bool,
}

#[cfg(test)]
impl Example for Food {
    fn examples() -> Vec<Self> {
        vec![Self {
            taste: 50,
            energy: 1000,
            purified: true,
        }]
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
    /// If this `ItemInst` is created from extending an existed registry then its id will be here.
    pub origin: Option<ID>,
    pub name: String,
    pub descr: String,
    /// In milimetres.
    pub length: i32,
    /// In mililitres.
    pub volume: i32,
    /// In grams.
    pub weight: i32,
    /// If the item is opaque.
    pub opaque: bool,
    /// In the smallest currency unit, like 1 USD cent.
    pub price: i32,
    pub feature: HashSet<Feature>,
    pub ext_info: Vec<String>,
    /// Detailed class, like weapon and armor.
    pub spec: ItemSpec,
}

#[cfg(test)]
impl Example for ItemInst {
    fn examples() -> Vec<Self> {
        ItemSpec::examples()
            .into_iter()
            .map(|s| Self {
                origin: None,
                name: "Example Item Instance".to_string(),
                descr: DEBUG_DESCR.to_string(),
                length: 114,
                volume: 514,
                weight: 514,
                opaque: true,
                price: 1919810,
                feature: HashSet::new(),
                ext_info: vec![],
                spec: s,
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
    use crate::json;
    for i in Item::examples() {
        println!("{}", json(&i).unwrap());
    }
}
