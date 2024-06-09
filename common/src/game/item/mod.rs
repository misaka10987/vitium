pub mod armor;
pub mod container;
pub mod edible;
pub mod melee;
pub mod ranged;
#[cfg(test)]
mod test;

use self::{armor::Armor, container::Container, edible::Edible, melee::Melee, ranged::Ranged};

use crate::{
    delta::Delta,
    t_recs::{reg::RegTab, store::btree::BTreeStore, Compon, Entity, Regis},
    with_btree_store, with_reg, Id, UId,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, error::Error, path::Path};
use tracing::debug;

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub base: BaseItem,
    pub armor: Option<Compon<Armor>>,
    pub container: Option<Compon<Container>>,
    pub edible: Option<Compon<Edible>>,
    pub melee: Option<Compon<Melee>>,
    pub ranged: Option<Compon<Ranged>>,
}

impl Entity for Item {
    type Base = BaseItem;
}

/// Basic information of an item is stored here.
#[derive(Clone, Serialize, Deserialize)]
pub struct BaseItem {
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
    pub flag: HashSet<Id>,
}

impl Regis for BaseItem {
    type Data = ();
}

pub struct ItemStore {
    r_base: &'static RegTab<BaseItem>,
    r_armor: &'static RegTab<Armor>,
    r_container: &'static RegTab<Container>,
    r_edible: &'static RegTab<Edible>,
    r_melee: &'static RegTab<Melee>,
    r_ranged: &'static RegTab<Ranged>,
    base: BTreeStore<Item>,
    armor: BTreeStore<Item, Armor>,
    container: BTreeStore<Item, Container>,
    edible: BTreeStore<Item, Edible>,
    melee: BTreeStore<Item, Melee>,
    ranged: BTreeStore<Item, Ranged>,
}

with_reg!(ItemStore, r_base, BaseItem);
with_reg!(ItemStore, r_armor, Armor);
with_reg!(ItemStore, r_container, Container);
with_reg!(ItemStore, r_edible, Edible);
with_reg!(ItemStore, r_melee, Melee);
with_reg!(ItemStore, r_ranged, Ranged);
with_btree_store!(ItemStore, base, Item);
with_btree_store!(ItemStore, armor, Item, Armor);
with_btree_store!(ItemStore, container, Item, Container);
with_btree_store!(ItemStore, edible, Item, Edible);
with_btree_store!(ItemStore, melee, Item, Melee);
with_btree_store!(ItemStore, ranged, Item, Ranged);

impl Delta for ItemStore {
    type Item = PackItemStore;

    fn calc(&mut self) -> impl Iterator<Item = Self::Item> {
        [PackItemStore {
            base: self.base.calc().collect(),
            armor: self.armor.calc().collect(),
            container: self.container.calc().collect(),
            edible: self.edible.calc().collect(),
            melee: self.melee.calc().collect(),
            ranged: self.ranged.calc().collect(),
        }]
        .into_iter()
    }

    fn diff(&self) -> impl Iterator<Item = Self::Item> {
        [PackItemStore {
            base: self.base.diff().collect(),
            armor: self.armor.diff().collect(),
            container: self.container.diff().collect(),
            edible: self.edible.diff().collect(),
            melee: self.melee.diff().collect(),
            ranged: self.ranged.diff().collect(),
        }]
        .into_iter()
    }

    fn apply(&mut self, delta: impl Iterator<Item = Self::Item>) {
        for i in delta {
            let PackItemStore {
                base,
                armor,
                container,
                edible,
                melee,
                ranged,
            } = i;
            self.base.apply(base.into_iter());
            self.armor.apply(armor.into_iter());
            self.container.apply(container.into_iter());
            self.edible.apply(edible.into_iter());
            self.melee.apply(melee.into_iter());
            self.ranged.apply(ranged.into_iter());
        }
    }
}

type Pack<T> = Vec<(UId<Item>, Option<Compon<T>>)>;

#[derive(Clone, Serialize, Deserialize)]
pub struct PackItemStore {
    pub base: Pack<BaseItem>,
    pub armor: Pack<Armor>,
    pub container: Pack<Container>,
    pub edible: Pack<Edible>,
    pub melee: Pack<Melee>,
    pub ranged: Pack<Ranged>,
}

#[derive(Clone, Copy)]
pub struct ItemReg {
    pub base: &'static RegTab<BaseItem>,
    pub armor: &'static RegTab<Armor>,
    pub container: &'static RegTab<Container>,
    pub edible: &'static RegTab<Edible>,
    pub melee: &'static RegTab<Melee>,
    pub ranged: &'static RegTab<Ranged>,
}

impl ItemReg {
    /// Unsafely drops the whole `ItemReg`.
    pub unsafe fn drop(&self) {
        let ItemReg {
            base,
            armor,
            container,
            edible,
            melee,
            ranged,
        } = self;
        RegTab::drop(base);
        RegTab::drop(armor);
        RegTab::drop(container);
        RegTab::drop(edible);
        RegTab::drop(melee);
        RegTab::drop(ranged);
    }
}

with_reg!(ItemReg, base, BaseItem);
with_reg!(ItemReg, armor, Armor);
with_reg!(ItemReg, container, Container);
with_reg!(ItemReg, edible, Edible);
with_reg!(ItemReg, melee, Melee);
with_reg!(ItemReg, ranged, Ranged);

pub struct ItemRegLoader {
    base: RegTab<BaseItem>,
    armor: RegTab<Armor>,
    container: RegTab<Container>,
    edible: RegTab<Edible>,
    melee: RegTab<Melee>,
    ranged: RegTab<Ranged>,
}

impl Default for ItemRegLoader {
    fn default() -> Self {
        Self {
            base: Default::default(),
            armor: Default::default(),
            container: Default::default(),
            edible: Default::default(),
            melee: Default::default(),
            ranged: Default::default(),
        }
    }
}

impl ItemRegLoader {
    /// Create item registry **without** builtins.
    pub fn new() -> Self {
        Self {
            base: RegTab::new(),
            armor: RegTab::new(),
            container: RegTab::new(),
            edible: RegTab::new(),
            melee: RegTab::new(),
            ranged: RegTab::new(),
        }
    }

    pub fn merge(&mut self, other: Self) -> impl Iterator<Item = Id> + '_ {
        let Self {
            base,
            armor,
            container,
            edible,
            melee,
            ranged,
        } = self;
        base.merge(other.base)
            .map(|x| omit(x))
            .chain(armor.merge(other.armor).map(|x| omit(x)))
            .chain(container.merge(other.container).map(|x| omit(x)))
            .chain(edible.merge(other.edible).map(|x| omit(x)))
            .chain(melee.merge(other.melee).map(|x| omit(x)))
            .chain(ranged.merge(other.ranged).map(|x| omit(x)))
    }

    pub fn load(path2mod: impl AsRef<Path>, modname: &str) -> Result<Self, Box<dyn Error>> {
        let p = path2mod.as_ref().join("reg/item");
        debug!("loading `Item` registries from {}", modname);
        Ok(Self {
            base: RegTab::load(p.join("base"))?,
            armor: RegTab::load(p.join("armor"))?,
            container: RegTab::load(p.join("container"))?,
            edible: RegTab::load(p.join("edible"))?,
            melee: RegTab::load(p.join("melee"))?,
            ranged: RegTab::load(p.join("ranged"))?,
        })
    }

    pub fn load_more(
        &mut self,
        path2mod: impl AsRef<Path>,
        modname: &str,
    ) -> Result<&mut Self, Box<dyn Error>> {
        let more = Self::load(&path2mod, modname)?;
        for id in self.merge(more) {
            println!("module {} overrides reg[id={}]", modname, id);
        }
        Ok(self)
    }

    pub fn leak(self) -> ItemReg {
        ItemReg {
            base: self.base.leak(),
            armor: self.armor.leak(),
            container: self.container.leak(),
            edible: self.edible.leak(),
            melee: self.melee.leak(),
            ranged: self.ranged.leak(),
        }
    }
}

fn omit<T>(x: (Id, T)) -> Id {
    x.0
}

impl Default for RegTab<BaseItem> {
    fn default() -> Self {
        Self(Default::default())
    }
}
