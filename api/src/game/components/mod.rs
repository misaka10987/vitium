pub mod armor;
pub mod char;
pub mod container;
pub mod edible;
pub mod hp;
pub mod item;
pub mod melee;
pub mod pos;
mod prelude;
pub mod ranged;

use bevy_ecs::{entity::Entity, world::World};

pub use prelude::*;

pub fn serialize_entity(entity: Entity, world: &World) -> String {
    let mut value = serde_json::Map::new();
    if let Some(comp) = world.get::<Item>(entity) {
        value.insert("Item".into(), serde_json::to_value(comp).unwrap());
    }
    serde_json::to_string(&value).unwrap()
}
