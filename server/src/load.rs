use std::{io, path::Path};

use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::fs;
use tracing::warn;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    JSON(#[from] serde_json::Error),
    #[error(transparent)]
    TOML(#[from] toml::de::Error),
}

pub async fn load_json<T: DeserializeOwned>(p: impl AsRef<Path>) -> Result<T, Error> {
    let s = fs::read(p).await?;
    let v = serde_json::from_slice(&s)?;
    Ok(v)
}

pub async fn try_load_json<T>(p: impl AsRef<Path>) -> T
where
    T: Default + DeserializeOwned,
{
    match load_json(&p).await {
        Ok(v) => v,
        Err(e) => {
            warn!("load {} failed: {e}", p.as_ref().display());
            Default::default()
        }
    }
}

pub async fn load_toml<T: DeserializeOwned>(p: impl AsRef<Path>) -> Result<T, Error> {
    let s = fs::read_to_string(p).await?;
    let v = toml::from_str(&s)?;
    Ok(v)
}

pub async fn try_load_toml<T>(p: impl AsRef<Path>) -> T
where
    T: Default + DeserializeOwned,
{
    match load_toml(&p).await {
        Ok(v) => v,
        Err(e) => {
            warn!("load {} failed: {e}", p.as_ref().display());
            Default::default()
        }
    }
}
