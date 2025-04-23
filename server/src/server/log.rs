use std::{collections::HashMap, str::FromStr, sync::Mutex};

use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_subscriber::{
    filter::Targets,
    layer::SubscriberExt,
    reload::{self, Handle},
    util::SubscriberInitExt,
    Registry,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Config(HashMap<String, String>);

impl Default for Config {
    fn default() -> Self {
        Self(HashMap::from([("rustyline".into(), "INFO".into())]))
    }
}

pub struct LogModule {
    pub cfg: Config,
    reload: Mutex<Handle<Targets, Registry>>,
}

impl LogModule {
    pub fn new(cfg: Config) -> anyhow::Result<Self> {
        let mut entry = vec![];
        for (k, v) in &cfg.0 {
            let level = Level::from_str(&v)?;
            entry.push((k, level));
        }
        let filter = Targets::new()
            .with_default(Level::TRACE)
            .with_targets(entry);
        let (filter, reload) = reload::Layer::new(filter);
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
        let value = Self {
            cfg,
            reload: Mutex::new(reload),
        };
        Ok(value)
    }
}

pub trait LogServer {
    fn reload_logger(&self, setter: impl FnOnce(&mut Targets)) -> anyhow::Result<()>;
}

impl<T> LogServer for T
where
    T: AsRef<LogModule>,
{
    fn reload_logger(&self, setter: impl FnOnce(&mut Targets)) -> anyhow::Result<()> {
        self.as_ref().reload.lock().unwrap().modify(setter)?;
        Ok(())
    }
}
