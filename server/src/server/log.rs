use std::{collections::HashMap, str::FromStr, sync::Mutex};

use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_subscriber::{
    Registry,
    filter::Targets,
    layer::SubscriberExt,
    reload::{self, Handle},
    util::SubscriberInitExt,
};

use crate::Server;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogConfig(HashMap<String, String>);

impl Default for LogConfig {
    fn default() -> Self {
        Self(HashMap::from([("rustyline".into(), "INFO".into())]))
    }
}

pub struct LogModule {
    pub config: LogConfig,
    reload: Mutex<Handle<Targets, Registry>>,
}

impl LogModule {
    pub fn new(config: LogConfig) -> anyhow::Result<Self> {
        let mut entry = vec![];
        for (k, v) in &config.0 {
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
            config,
            reload: Mutex::new(reload),
        };
        Ok(value)
    }

    pub fn update(&self, setter: impl FnOnce(&mut Targets)) -> anyhow::Result<()> {
        self.reload.lock().unwrap().modify(setter)?;
        Ok(())
    }
}

impl Server {
    pub fn update_logger(&self, setter: impl FnOnce(&mut Targets)) -> anyhow::Result<()> {
        self.log.update(setter)
    }
}
