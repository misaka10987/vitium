pub use crate::load::load;
pub use crate::save::save;
use crate::{
    chara::chara,
    registry::{reg_item, reg_scene, reg_vehicle},
};
use axum::http::StatusCode;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use tokio::sync::{Mutex, MutexGuard};
use vitium_common::{
    act::{Act, Action},
    cmd::Command,
    item::Item,
    scene::Scene,
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

static TURN: Lazy<Mutex<i128>> = Lazy::new(|| Mutex::new(255));
/// Starts from `0`, defines how many turns has passed after the game starts.
pub async fn turn() -> MutexGuard<'static, i128> {
    TURN.lock().await
}

static ACT: Lazy<Mutex<VecDeque<Act>>> = Lazy::new(|| Mutex::new(VecDeque::<Act>::new()));
/// Act queue pushed by request handlers and poped by the game.
pub async fn act() -> MutexGuard<'static, VecDeque<Act>> {
    ACT.lock().await
}

/// Pushes `act` to the act queue waiting for process.
pub async fn push_act(raw_act: Act) -> StatusCode {
    self::act()
        .await
        .push_back(raw_act.clone().set_uid(gen_uid().await).to_owned());
    tokio::spawn(update());
    StatusCode::ACCEPTED
}

static ON_GAME: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub async fn on() -> bool {
    *ON_GAME.lock().await
}

static TERM_GAME: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

/// This sends a terminate signal to the game instance.
pub async fn term_game() {
    *ON_GAME.lock().await = false;
    *TERM_GAME.lock().await = true;
}

/// Process server command.
pub async fn cmd(command: Command) {
    match command {
        Command::Hello => println!("[cmd] Hello, world!"),
    }
}

/// Calculate all waiting requests.
async fn update() {
    while let Some(a) = act().await.pop_front() {
        if let Err(s) = proc(a).await {
            println!("invalid act: {}", s)
        }
    }
}

/// Internal `Act` process function.
async fn proc(mut act: Act) -> Result<(), String> {
    act.set_uid(gen_uid().await);
    if act.turn != *turn().await {
        return Err(format!("act[uid={}] non in correct turn", act.uid()));
    }
    match act.action {
        Action::Move(_) => todo!(),
        Action::Wield(_) => todo!(),
        Action::Cast(_) => todo!(),
        Action::Hello => hello(act).await,
    }
    Ok(())
}

/// Act's helloworld.
async fn hello(act: Act) {
    println!(
        "{}: \"Hello, world!\"",
        match chara().await.get(&act.chara) {
            Some(c) => c.name.clone(),
            None => format!("[!chara uid={}]", act.chara),
        }
    )
}
