use crate::{
    chara::chara,
    load::load,
    registry::{reg_item, reg_prof, reg_scene, reg_skill, reg_vehicle},
    save::save,
};
use axum::http::StatusCode;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use tokio::sync::{Mutex, MutexGuard};
use vitium_common::{
    act::{Act, Action},
    item::Item,
    scene::Scene,
    skill::{Prof, Skill},
    vehicle::Vehicle,
    UID,
};

static UID_ALLOC: Lazy<Mutex<i128>> = Lazy::new(|| Mutex::new(255));
/// The server allocates uid in time order.
/// `0..255` reserved.
pub async fn gen_uid() -> i128 {
    let mut now = UID_ALLOC.lock().await;
    *now += 1;
    *now
}

/// Returns new instance of `id`.
/// # Panic
/// Panics if `id` is not registered.
pub async fn item(id: &str) -> Item {
    reg_item()
        .await
        .get(id)
        .expect(&format!("item {} non registered", id))
        .inst()
        .set_uid(gen_uid().await)
        .to_owned()
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
        .set_uid(gen_uid().await)
        .to_owned()
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
        .set_uid(gen_uid().await)
        .to_owned()
}

/// Starts from `0`, defines how many turns has passed after the game starts.
static TURN: Lazy<Mutex<i128>> = Lazy::new(|| Mutex::new(255));

pub async fn turn() -> MutexGuard<'static, i128> {
    TURN.lock().await
}

/// Act queue pushed by request handlers and poped by the game.
static ACT: Lazy<Mutex<VecDeque<Act>>> = Lazy::new(|| Mutex::new(VecDeque::<Act>::new()));

pub async fn act() -> MutexGuard<'static, VecDeque<Act>> {
    ACT.lock().await
}

/// Pushes `act` to the act queue waiting for process.
pub async fn push_act(raw_act: Act) -> StatusCode {
    self::act()
        .await
        .push_back(raw_act.clone().set_uid(gen_uid().await).to_owned());
    StatusCode::ACCEPTED
}

static ON_GAME: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static TERM_GAME: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

/// This sends a terminate signal to the game instance.
pub async fn term_game() {
    *ON_GAME.lock().await = false;
    *TERM_GAME.lock().await = true;
}

/// The game main function.
pub async fn game() {
    *ON_GAME.lock().await = true;
    load("./save").await;
    let last_save: i128 = 0;
    loop {
        if *TERM_GAME.lock().await {
            break save("./save").await;
        }
        if last_save > 15 {
            save("./save").await;
        }
        if let Some(a) = act().await.pop_front() {
            proc(a.action, a.chara).await
        }
    }
}

async fn proc(action: Action, character: i128) {
    match action {
        Action::Move(_) => todo!(),
        Action::Wield(_) => todo!(),
        Action::Cast(_) => todo!(),
        Action::Hello => hello(character).await,
    }
}

async fn hello(character: i128) {
    println!(
        "{}: \"Hello, world!\"",
        match chara().await.get(&character) {
            Some(c) => c.name.clone(),
            None => format!("[character uid={} non exist]", character),
        }
    )
}
