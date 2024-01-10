use crate::{age::CAge, dice::CDice, CVector, Rust, C};
use std::ffi::c_char;
use vitium_common::{
    age::Age,
    item::{Armor, BodyPart, Item, OtherItem, Price, Species, Weapon},
    json::{json, obj},
    UID,
};

#[repr(transparent)]
pub struct CPrice(CVector<C_CAge_Cu64>);
impl C<CPrice> for Price {
    fn c(&self) -> CPrice {
        CPrice(self.c())
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct C_CAge_Cu64 {
    v1: CAge,
    v2: u64,
}
impl C<C_CAge_Cu64> for (Age, u64) {
    fn c(&self) -> C_CAge_Cu64 {
        let (a, p) = self;
        C_CAge_Cu64 { v1: a.c(), v2: *p }
    }
}
impl Rust<(Age, u64)> for C_CAge_Cu64 {
    unsafe fn rs(&self) -> (Age, u64) {
        (self.v1.rs(), self.v2)
    }
}

#[repr(C)]
/// Instance of weapon.
pub struct CWeapon {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: i64,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: *const c_char,
    /// Name dieplayed in game.
    pub name: *const c_char,
    /// Description displayed in game.
    pub descr: *const c_char,
    /// Age periods available.
    pub age: CVector<CAge>,
    /// Damage expression using dice, eg `1d4+1`.
    pub atk: CDice,
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
    pub price: CPrice,
}
impl C<CWeapon> for Weapon {
    fn c(&self) -> CWeapon {
        CWeapon {
            uid: self.uid() as i64,
            id: self.id.c(),
            name: self.name.c(),
            descr: self.descr.c(),
            age: self.age.c(),
            atk: self.atk.c(),
            rng: self.rng,
            pntr: self.pntr,
            per_turn: self.per_turn,
            charge: self.per_turn,
            load: self.load,
            price: self.price.c(),
        }
    }
}
impl Rust<Weapon> for CWeapon {
    unsafe fn rs(&self) -> Weapon {
        Weapon {
            uid: self.uid as i128,
            id: self.id.rs(),
            name: self.name.rs(),
            descr: self.descr.rs(),
            age: self.age.rs(),
            atk: self.atk.rs(),
            rng: self.rng,
            pntr: self.pntr,
            per_turn: self.per_turn,
            charge: self.charge,
            load: self.load,
            price: self.price.0.rs(),
        }
    }
}

#[repr(transparent)]
pub struct CBodyPart(*const c_char);
impl C<CBodyPart> for BodyPart {
    fn c(&self) -> CBodyPart {
        CBodyPart(self.c())
    }
}
impl Rust<BodyPart> for CBodyPart {
    unsafe fn rs(&self) -> BodyPart {
        self.0.rs()
    }
}

/// Defines species for deciding if an armor is able to wear.
#[repr(C)]
pub enum CSpecies {
    /// Human-liked species.
    Human,
    /// Non human-liked species.
    NonHuman,
    /// Let host decide if able to wear.
    Else(*const c_char),
}
impl C<CSpecies> for Species {
    fn c(&self) -> CSpecies {
        match self {
            Species::Human => CSpecies::Human,
            Species::NonHuman => CSpecies::NonHuman,
            Species::Else(e) => CSpecies::Else(e.c()),
        }
    }
}
impl Rust<Species> for CSpecies {
    unsafe fn rs(&self) -> Species {
        match self {
            CSpecies::Human => Species::Human,
            CSpecies::NonHuman => Species::NonHuman,
            CSpecies::Else(e) => Species::Else(e.rs()),
        }
    }
}

/// Instance of armor.
#[repr(C)]
pub struct CArmor {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: i64,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: *const c_char,
    /// Name dieplayed in game.
    pub name: *const c_char,
    /// Description displayed in game.
    pub descr: *const c_char,
    /// Age periods available.
    pub age: CVector<CAge>,
    /// Damage
    pub def: *const c_char,
    /// Covered body parts.
    pub cover: CVector<CBodyPart>,
    /// Species able to wear this armor.
    pub species: CSpecies,
    /// Whether resists penetration.
    pub rerist_pntr: bool,
    /// Price in different time periods.
    pub price: CPrice,
}
impl C<CArmor> for Armor {
    fn c(&self) -> CArmor {
        CArmor {
            uid: self.uid() as i64,
            id: self.id.c(),
            name: self.name.c(),
            descr: self.descr.c(),
            age: self.age.c(),
            def: self.def.c(),
            cover: self.cover.c(),
            species: self.species.c(),
            rerist_pntr: self.rerist_pntr,
            price: self.price.c(),
        }
    }
}
impl Rust<Armor> for CArmor {
    unsafe fn rs(&self) -> Armor {
        Armor {
            uid: self.uid as i128,
            id: self.id.rs(),
            name: self.name.rs(),
            descr: self.descr.rs(),
            age: self.age.rs(),
            def: self.def.rs(),
            cover: self.cover.rs(),
            species: self.species.rs(),
            rerist_pntr: self.rerist_pntr,
            price: self.price.0.rs(),
        }
    }
}

#[repr(C)]
pub struct COtherItem {
    /// Unique in-game id generated automatically. Set to `0` to let the program generate.
    pub uid: i64,
    /// String ID for `Item`, must be unique.
    ///
    /// Any char that is allowed in a valid filename is allowed here, like `-`.
    pub id: *const c_char,
    /// Name displayed in game.
    pub name: *const c_char,
    /// Description displayed in game.
    pub descr: *const c_char,
}
impl C<COtherItem> for OtherItem {
    fn c(&self) -> COtherItem {
        COtherItem {
            uid: self.uid() as i64,
            id: self.id.c(),
            name: self.name.c(),
            descr: self.descr.c(),
        }
    }
}
impl Rust<OtherItem> for COtherItem {
    unsafe fn rs(&self) -> OtherItem {
        OtherItem {
            uid: self.uid as i128,
            id: self.id.rs(),
            name: self.name.rs(),
            descr: self.descr.rs(),
        }
    }
}

#[repr(C)]
pub enum CItem {
    Weapon(CWeapon),
    Armor(CArmor),
    Other(COtherItem),
}
impl C<CItem> for Item {
    fn c(&self) -> CItem {
        match self {
            Item::Weapon(i) => CItem::Weapon(i.c()),
            Item::Armor(i) => CItem::Armor(i.c()),
            Item::Other(i) => CItem::Other(i.c()),
        }
    }
}
impl Rust<Item> for CItem {
    unsafe fn rs(&self) -> Item {
        match self {
            CItem::Weapon(i) => Item::Weapon(i.rs()),
            CItem::Armor(i) => Item::Armor(i.rs()),
            CItem::Other(i) => Item::Other(i.rs()),
        }
    }
}

#[no_mangle]
extern "C" fn c_obj_CItem(json: *const c_char) -> CItem {
    unsafe { obj::<Item>(&json.rs()).c() }
}

#[no_mangle]
extern "C" fn c_json_CItem(obj: CItem) -> *const c_char {
    unsafe { json(obj.rs()).c() }
}
