use std::ffi::{c_char, CStr};
use vitium_common::{
    age::Age,
    item::{Armor, BodyPart, Item, OtherItem, Species, Weapon},
    json::obj,
    UID,
};

use crate::{age::CAge, dice::CDice, ptr, CVector, C};

pub type CPrice = CVector;

//#[allow(non_camel_case_types)]
#[repr(C)]
pub struct C_Tuple_CAge_Cu64 {
    v_1: CAge,
    v_2: u64,
}

impl C<C_Tuple_CAge_Cu64> for (Age, u64) {
    fn c(&self) -> C_Tuple_CAge_Cu64 {
        let (a, p) = self;
        C_Tuple_CAge_Cu64 {
            v_1: a.c(),
            v_2: *p,
        }
    }
}

#[repr(C)]
pub struct CWeapon {
    pub uid: i64,
    pub id: *const c_char,
    pub name: *const c_char,
    pub descr: *const c_char,
    pub age: CVector,
    pub atk: CDice,
    pub rng: u32,
    pub pntr: bool,
    pub per_turn: u8,
    pub charge: u8,
    pub load: u8,
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

pub type CBodyPart = *const c_char;

#[repr(C)]
pub enum CSpecies {
    Human,
    NonHuman,
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
    pub age: CVector,
    /// Damage
    pub def: *const c_char,
    /// Covered body parts.
    pub cover: CVector,
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

#[repr(C)]
pub enum CItem {
    Weapon(*const CWeapon),
    Armor(*const CArmor),
    Other(*const COtherItem),
}
impl C<CItem> for Item {
    fn c(&self) -> CItem {
        match self {
            Item::Weapon(i) => CItem::Weapon(ptr(&i.c())),
            Item::Armor(i) => CItem::Armor(ptr(&i.c())),
            Item::Other(i) => CItem::Other(ptr(&i.c())),
        }
    }
}

pub type CInventory = CVector;

#[no_mangle]
extern "C" fn c_obj_CItem(json: *const c_char) -> CItem {
    let cstr: &CStr = unsafe { CStr::from_ptr(json) };
    let slice = cstr
        .to_str()
        .expect("fn c_obj_CItem got invalid C const char*");
    obj::<Item>(slice).c()
}
