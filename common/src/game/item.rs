use super::DmgType;
use crate::{dice::Dice, ID, UID};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum Item {
    Basic(BaseItem),
    Container(Container),
    Melee(Melee),
    Ranged(Ranged),
    Armor(Armor),
    Food(Food),
    Other(OtherItem),
}

impl AsRef<BaseItem> for Item {
    fn as_ref(&self) -> &BaseItem {
        &self.deref()
    }
}

impl Deref for Item {
    type Target = BaseItem;

    fn deref(&self) -> &Self::Target {
        match self {
            Item::Basic(i) => i,
            Item::Container(i) => i,
            Item::Melee(i) => i,
            Item::Ranged(i) => i,
            Item::Armor(i) => i,
            Item::Food(i) => i,
            Item::Other(i) => i,
        }
    }
}

impl AsRef<Option<ID>> for Item {
    fn as_ref(&self) -> &Option<ID> {
        self.deref().as_ref()
    }
}

/// Basic information of an item is stored here.
///
/// # NOTA BENE
///
/// Invoking `.clone()` on this struct does **NOT** produce another instance with completely the same fields.
/// Instead its `.reg` will be replaced by `None`.
/// This is to make `Cow`s happy when implementing registeries.
#[derive(Serialize, Deserialize)]
pub struct BaseItem {
    reg: Option<ID>,
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
    /// Extended information displayed.
    pub ext_info: Vec<String>,
    /// Flags.
    pub flag: HashSet<ID>,
}

impl Clone for BaseItem {
    fn clone(&self) -> Self {
        Self {
            reg: None,
            name: self.name.clone(),
            descr: self.descr.clone(),
            length: self.length.clone(),
            volume: self.volume.clone(),
            weight: self.weight.clone(),
            opaque: self.opaque.clone(),
            price: self.price.clone(),
            ext_info: self.ext_info.clone(),
            flag: self.flag.clone(),
        }
    }
}

impl AsRef<Option<ID>> for BaseItem {
    fn as_ref(&self) -> &Option<ID> {
        &self.reg
    }
}

impl AsRef<BaseItem> for BaseItem {
    fn as_ref(&self) -> &BaseItem {
        self
    }
}

impl BaseItem {
    /// If this `Item` is a registry then its id will be here.
    pub fn reg(&self) -> &Option<ID> {
        &self.reg
    }
    /// Unsafely change the registry id of this item.
    pub unsafe fn mut_reg(&mut self) -> &mut Option<ID> {
        &mut self.reg
    }
}

macro_rules! impl_item {
    ($t:ty) => {
        impl_item!($t, base);
    };
    ($t:ty,$f:ident) => {
        impl std::ops::Deref for $t {
            type Target = $crate::game::item::BaseItem;

            fn deref(&self) -> &Self::Target {
                &self.$f
            }
        }

        impl std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$f
            }
        }

        impl std::convert::AsRef<$crate::game::item::BaseItem> for $t {
            fn as_ref(&self) -> &$crate::game::item::BaseItem {
                self.deref().as_ref()
            }
        }
    };
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Container {
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
    pub inside: Vec<UID<Item>>,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Food {
    pub base: BaseItem,
    pub taste: i8,
    pub energy: i32,
    pub purified: bool,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ArmorLayer {
    pub mat: ID,
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

impl_item!(Container);
impl_item!(Melee);
impl_item!(Ranged);
impl_item!(Armor);
impl_item!(Food);
impl_item!(OtherItem);

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use crate::{
        game::{item::OtherItem, DmgType},
        json,
        test::*,
        ID, UID,
    };

    use super::{Armor, ArmorLayer, BaseItem, Container, Food, Item, Melee, Ranged, Species};

    #[test]
    fn view_json() {
        println!("{}", json(&Item::examples()).unwrap());
    }

    impl Example for Item {
        fn examples() -> Vec<Self> {
            vec![
                Item::Basic(BaseItem::example()),
                Item::Container(Container::example()),
                Item::Melee(Melee::example()),
                Item::Ranged(Ranged::example()),
                Item::Armor(Armor::example()),
                Item::Food(Food::example()),
                Item::Other(OtherItem::example()),
            ]
        }
    }

    impl Example for Container {
        fn examples() -> Vec<Self> {
            vec![Self {
                base: BaseItem::example(),
                time_cost: 1000,
                length: 114,
                volume: 514,
                weight: 1919,
                waterproof: false,
                inside: vec![UID::new(114), UID::new(514)],
            }]
        }
    }

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

    impl Example for Species {
        fn examples() -> Vec<Self> {
            vec![Self::Human, Self::NonHuman, Self::Else("cat".to_string())]
        }
    }

    impl Example for ArmorLayer {
        fn examples() -> Vec<Self> {
            let mut cover = HashSet::new();
            cover.extend(ID::examples());
            vec![Self {
                mat: ID::example(),
                cover,
                rate: 95,
                thickness: 3000,
            }]
        }
    }

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
                flag: HashSet::new(),
            }]
        }
    }

    impl Example for OtherItem {
        fn examples() -> Vec<Self> {
            vec![Self {
                base: BaseItem::example(),
            }]
        }
    }
}
