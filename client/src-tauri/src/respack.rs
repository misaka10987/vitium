use std::{collections::HashMap, path::PathBuf};

use dirs::data_dir;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use vitium_api::{
    game::{Item, Prof, Terra},
    reg::{HasRegTab, Id},
};

use crate::CFG;

#[derive(Clone, Serialize, Deserialize)]
pub struct ResPack {
    pub id: String,
    pub author: String,
    pub name: String,
    pub descr: String,
}

static TEXTURE_TERRA: Lazy<HashMap<Id<Terra>, PathBuf>> = Lazy::new(|| {
    let list = &CFG.respack;
    for x in Terra::reg_tab().iter() {
        let y = Id::<Terra>::new(x.key()).modname();
    }
    let x = 1;
    HashMap::new()
});

fn init() {
    let dir = data_dir().unwrap().join("vitium").join("respack");
    let v: Vec<_> = dir.read_dir().unwrap().map(|r| r.unwrap()).collect();
    for i in v {}
}

fn f() {}
