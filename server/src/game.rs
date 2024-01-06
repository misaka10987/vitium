use crate::registry::{reg_item, reg_prof, reg_scene, reg_skill, reg_vehicle};
use vitium_common::{
    item::Item,
    scene::Scene,
    skill::{Prof, Skill},
    vehicle::Vehicle,
};

/// Returns new instance of `id`.
/// # Panic
/// Panics if `id` is not registered.
pub async fn item(id: &str) -> Item {
    reg_item()
        .await
        .get(id)
        .expect(&format!("item {} non registered", id))
        .inst()
}

/// Returns new instance of `id`.
/// # Panic
/// Panics if `id` is not registered.
pub async fn skill(id: &str) -> Skill {
    reg_skill()
        .await
        .get(id)
        .expect(&format!("skill {} non registered", id))
        .inst()
}

/// Returns new instance of `id`.
/// # Panic
/// Panics if `id` is not registered.
pub async fn prof(id: &str) -> Prof {
    reg_prof()
        .await
        .get(id)
        .expect(&format!("profession {} non registered", id))
        .inst()
}

/// Returns new instance of `id`.
/// # Panic
/// Panics if `id` is not registered.
pub async fn scene(id: &str) -> Scene {
    reg_scene()
        .await
        .get(id)
        .expect(&format!("scenario {} non registered", id))
        .inst()
}

/// Returns new instance of `id`.
/// # Panic
/// Panics if `id` is not registered.
pub async fn vehicle(id: &str) -> Vehicle {
    reg_vehicle()
        .await
        .get(id)
        .expect(&format!("item {} non registered", id))
        .inst()
}
