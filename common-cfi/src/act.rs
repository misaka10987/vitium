use crate::C;
use std::ffi::c_char;
use vitium_common::{
    act::{Act, Action, Cast},
    UID,
};

pub struct CCast {
    pub skill: *const c_char,
    pub object: u64,
}
impl C<CCast> for Cast {
    fn c(&self) -> CCast {
        CCast {
            skill: self.skill.c(),
            object: self.object.c(),
        }
    }
}

pub enum CAction {
    Move(u64),
    Wield(u64),
    Cast(CCast),
    Hello,
}
impl C<CAction> for Action {
    fn c(&self) -> CAction {
        match self {
            Action::Move(a) => CAction::Move(a.c()),
            Action::Wield(a) => CAction::Wield(a.c()),
            Action::Cast(a) => CAction::Cast(a.c()),
            Action::Hello => CAction::Hello,
        }
    }
}

pub struct CAct {
    pub uid: u64,
    pub chara: u64,
    pub turn: u64,
    pub action: CAction,
}
impl C<CAct> for Act {
    fn c(&self) -> CAct {
        CAct {
            uid: self.uid().c(),
            chara: self.chara.c(),
            turn: self.turn.c(),
            action: self.action.c(),
        }
    }
}