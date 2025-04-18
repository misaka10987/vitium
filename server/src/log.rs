use std::sync::Mutex;

use static_init::dynamic;
use tracing::Level;
use tracing_subscriber::{
    filter::Targets,
    layer::SubscriberExt,
    reload::{self, Handle},
    util::SubscriberInitExt,
    Registry,
};

#[dynamic]
pub static FILTER: Mutex<Handle<Targets, Registry>> = {
    let filter = Targets::new()
        .with_default(Level::TRACE)
        .with_target("rustyline", Level::INFO);
    let (filter, reload) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    Mutex::new(reload)
};
