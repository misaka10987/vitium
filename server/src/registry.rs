use once_cell::sync::Lazy;
use tokio::{
    fs::read_to_string,
    sync::{Mutex, MutexGuard},
};
use vitium_common::{
    item::Item,
    json::obj,
    registry::{RegTable, Regis},
    scene::Scene,
    skill::{Prof, Skill},
    vehicle::Vehicle,
    ID,
};

type REG<T> = Lazy<Mutex<RegTable<T>>>;
macro_rules! reg {
    () => {
        Lazy::new(|| Mutex::new(RegTable::new()))
    };
}
static REG_ITEM: REG<Item> = reg!();
static REG_SKILL: REG<Skill> = reg!();
static REG_PROF: REG<Prof> = reg!();
static REG_SCENE: REG<Scene> = reg!();
static REG_VEHICLE: REG<Vehicle> = reg!();

pub async fn reg_item() -> MutexGuard<'static, RegTable<Item>> {
    REG_ITEM.lock().await
}
pub async fn reg_skill() -> MutexGuard<'static, RegTable<Skill>> {
    REG_SKILL.lock().await
}
pub async fn reg_prof() -> MutexGuard<'static, RegTable<Prof>> {
    REG_PROF.lock().await
}
pub async fn reg_scene() -> MutexGuard<'static, RegTable<Scene>> {
    REG_SCENE.lock().await
}
pub async fn reg_vehicle() -> MutexGuard<'static, RegTable<Vehicle>> {
    REG_VEHICLE.lock().await
}

/// Ganked by compiler, therefore rewritten using macro.
// async fn load_reg<'a, T>(reg: MutexGuard<'static, RegTable<T>>, path: &str)
// where
//     T: serde::de::Deserialize<'a> + ID + Clone,
// {
//     let buf = read_to_string(path)
//         .await
//         .expect(&format!("io error with {}", path));
//     let buf = obj::<Vec<T>>(&buf);
//     //let mut reg = reg_item().await;
//     let mut reg = reg;
//     for i in buf {
//         if reg.contains_key(&i.id()) {
//             panic!("{} tried to register an already-exist id", path)
//         }
//         reg.insert(i.id(), Regis::Dynamic(i));
//     }
// }
macro_rules! load_reg {
    ($type:ty,$reg:expr,$path:expr) => {
        let buf = read_to_string($path)
            .await
            .expect(&format!("io error with {}", $path));
        let mut reg = $reg;
        for i in obj::<Vec<$type>>(&buf) {
            if reg.contains_key(&i.id()) {
                panic!("{} tried to register an existing id", $path)
            }
            reg.insert(i.id(), Regis::Dynamic(i));
        }
    };
}

pub async fn load(mod_root: Vec<&str>) {
    for rt in mod_root {
        load_reg!(Item, reg_item().await, format!("{}/reg/item.json", rt));
        load_reg!(Skill, reg_skill().await, format!("{}/reg/skill.json", rt));
        load_reg!(Prof, reg_prof().await, format!("{}/reg/prof.json", rt));
        load_reg!(Scene, reg_scene().await, format!("{}/reg/scene.json", rt));
        load_reg!(
            Vehicle,
            reg_vehicle().await,
            format!("{}/reg/vehicle.json", rt)
        );
        // let buf = read_to_string(format!("{}/reg/item.json", rt))
        //     .await
        //     .expect(&format!("io error with {}/", rt));
        // let mut reg = reg_item().await;
        // for i in obj::<Vec<Item>>(&buf) {
        //     if reg.contains_key(&i.id()) {
        //         panic!("{}/reg/item.json tried to register an already-exist id", rt)
        //     }
        //     reg.insert(i.id(), Regis::Dynamic(i));
        // }
    }
    todo!()
}
