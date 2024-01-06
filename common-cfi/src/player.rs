use crate::C;
use std::{ffi::c_char, ptr::null};
use vitium_common::player::{Player, Token};

#[repr(C)]
pub struct CPlayer {
    pub id: *const c_char,
    pub name: *const c_char,
    pub profile: *const c_char,
}

impl C<CPlayer> for Player {
    fn c(&self) -> CPlayer {
        CPlayer {
            id: self.id.c(),
            name: self.name.c(),
            profile: match &self.profile {
                Some(s) => s.c(),
                None => null(),
            },
        }
    }
}

#[repr(C)]
pub struct CToken {
    pub id: *const c_char,
    pub pswd: *const c_char,
}

impl C<CToken> for Token {
    fn c(&self) -> CToken {
        CToken {
            id: self.id.c(),
            pswd: self.pswd.c(),
        }
    }
}
