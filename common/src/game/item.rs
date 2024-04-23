use super::DmgType;
use crate::{dice::Dice, ID};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Serialize, Deserialize)]
pub enum Item<'a> {
    Container(Cow<'a, Container<'a>>),
    Melee(Cow<'a, Melee>),
    Ranged(Cow<'a, Ranged>),
    Armor(Cow<'a, Armor>),
    Food(Cow<'a, Food>),
    OtherItem(Cow<'a, OtherItem>),
}

impl<'a> Deref for Item<'a> {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        match self {
            Item::Container(i) => i,
            Item::Melee(i) => i,
            Item::Ranged(i) => i,
            Item::Armor(i) => i,
            Item::Food(i) => i,
            Item::OtherItem(i) => i,
        }
    }
}

impl<'a> DerefMut for Item<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Item::Container(i) => i.to_mut(),
            Item::Melee(i) => i.to_mut(),
            Item::Ranged(i) => i.to_mut(),
            Item::Armor(i) => i.to_mut(),
            Item::Food(i) => i.to_mut(),
            Item::OtherItem(i) => i.to_mut(),
        }
    }
}

impl<'a> AsRef<Option<ID>> for Item<'a> {
    fn as_ref(&self) -> &Option<ID> {
        self.deref().as_ref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BaseItem {
    /// If this `Item` is a registry then its id will be here.
    pub reg: Option<ID>,
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
    pub ext_info: Vec<String>,
}

impl AsRef<Option<ID>> for BaseItem {
    fn as_ref(&self) -> &Option<ID> {
        &self.reg
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Container<'a> {
    pub base: BaseItem,
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
    pub inside: Vec<Item<'a>>,
}

impl<'a> Deref for Container<'a> {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> DerefMut for Container<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Melee {
    pub base: BaseItem,
    pub atk: HashMap<DmgType, Dice>,
    /// In milimetres.
    pub rng: i32,
    pub one_hand: bool,
    pub skill: HashSet<ID>,
    pub mart: HashSet<ID>,
}

impl Deref for Melee {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Melee {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Ranged {
    pub base: BaseItem,
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

impl Deref for Ranged {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Ranged {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Food {
    pub base: BaseItem,
    pub taste: i8,
    pub energy: i32,
    pub purified: bool,
}

impl Deref for Food {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Food {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// Instance of armor.
#[derive(Clone, Serialize, Deserialize)]
pub struct Armor {
    pub base: BaseItem,
    /// Damage
    pub def: Dice,
    /// Species able to wear this armor.
    pub species: Species,
    pub layer: Vec<ArmorLayer>,
}

impl Deref for Armor {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Armor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArmorLayer {
    pub material: ID,
    pub cover: HashSet<ID>,
    pub rate: u16,
    pub thickness: u16,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct OtherItem {
    pub base: BaseItem,
}

impl Deref for OtherItem {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for OtherItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[cfg(test)]
use crate::test::*;

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
            base: BaseItem::example(),
            atk,
            rng: 514,
            one_hand: true,
            skill,
            mart,
        }]
    }
}

#[cfg(test)]
impl Example for Ranged {
    fn examples() -> Vec<Self> {
        let mut atk = HashMap::new();
        atk.insert(DmgType::System, "11d45+14".to_string());
        let mut charge = HashSet::new();
        charge.extend(ID::examples());
        vec![Self {
            base: BaseItem::example(),
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

#[cfg(test)]
impl Example for Food {
    fn examples() -> Vec<Self> {
        vec![Self {
            base: BaseItem::example(),
            taste: 50,
            energy: 1000,
            purified: true,
        }]
    }
}

#[cfg(test)]
impl Example for Species {
    fn examples() -> Vec<Self> {
        vec![Self::Human, Self::NonHuman, Self::Else("cat".to_string())]
    }
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

#[cfg(test)]
impl Example for Armor {
    fn examples() -> Vec<Self> {
        Species::examples()
            .into_iter()
            .map(|s| Self {
                base: BaseItem::example(),
                def: "11d45+14".to_string(),
                species: s,
                layer: ArmorLayer::examples(),
            })
            .collect()
    }
}

#[cfg(test)]
impl Example for BaseItem {
    fn examples() -> Vec<Self> {
        vec![Self {
            reg: None,
            name: "example item".to_string(),
            descr: DEBUG_DESCR.to_string(),
            length: 114,
            volume: 514,
            weight: 114,
            opaque: true,
            price: 514,
            ext_info: vec![],
        }]
    }
}
