use std::collections::HashSet;

use crate::{
    t_recs::Compon,
    test::{Example, DEBUG_DESCR},
};

use super::{BaseItem, Item};

impl Example for BaseItem {
    fn examples() -> Vec<Self> {
        vec![Self {
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

impl Example for Item {
    fn examples() -> Vec<Self> {
        vec![Self {
            base: BaseItem::example(),
            armor: Some(Compon::example()),
            container: Some(Compon::example()),
            edible: Some(Compon::example()),
            melee: Some(Compon::example()),
            ranged: Some(Compon::example()),
        }]
    }
}

#[test]
fn view_json() {
    println!("{}", crate::json(&Item::example()).unwrap());
}
