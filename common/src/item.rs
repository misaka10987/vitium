use crate::{age::Age, dice::Dice, util::Envelop};
use serde_derive::{Deserialize, Serialize};
pub type Price = Vec<(Age, u64)>;

/// Instance of weapon.
#[derive(Serialize, Deserialize, Clone)]
pub struct Weapon {
    pub id: String,
    pub name: String,
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

pub type BodyPart = String;

/// Defines species for deciding if an armor is able to wear.
pub enum ArmorSpecies {
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
    pub id: String,
    pub name: String,
    /// Age periods available.
    pub age: Vec<Age>,
    /// Damage
    pub def: Dice,
    /// Covered body parts.
    pub cover: Vec<BodyPart>,
    /// Whether resists penetration.
    pub rerist_pntr: bool,
    /// Price in different time periods.
    pub price: Price,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OtherItem {
    pub id: String,
    pub name: String,
    pub descr: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Other(OtherItem),
}

pub type Inventory = Vec<Envelop<Item>>;

#[test]
fn see_json() {
    use serde_json::to_string as json;
    let c = vec!["torso".to_string(), "head".to_string()];
    let i1 = Item::Weapon(Weapon {
        id: "example_id".to_string(),
        name: "example_name".to_string(),
        age: vec!["example_age".to_string()],
        atk: "11d45+14".to_string(),
        rng: 114514,
        pntr: true,
        per_turn: 2,
        charge: 5,
        load: 1,
        price: vec![("1920s".to_string(), 1919810)],
    });
    let i2 = Item::Armor(Armor {
        id: "example_id".to_string(),
        name: "example_name".to_string(),
        age: vec!["example_age".to_string()],
        def: "11d45+14".to_string(),
        cover: c,
        rerist_pntr: true,
        price: vec![("1920s".to_string(), 1919810)],
    });
    let i3 = Item::Other(OtherItem {
        id: "example_id".to_string(),
        name: "example_name".to_string(),
        descr: "This is description.".to_string(),
    });
    eprintln!("{}",json(&i1).unwrap());
    eprintln!("{}",json(&i2).unwrap());
    eprintln!("{}",json(&i3).unwrap());
}
