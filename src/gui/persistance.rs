use std::{fs as std_fs, path::PathBuf};

use async_std::fs;
use serde::{de::DeserializeOwned, Serialize};

pub trait Persistance {
    async fn save<T: Serialize>(items: T) -> Result<(), PersistError> {
        let save_string =
            serde_json::to_string(&items).map_err(|_| PersistError::Save(SaveError::Compose))?;

        fs::write(Self::config_path()?, save_string)
            .await
            .map_err(|_| PersistError::Save(SaveError::Write))?;

        Ok(())
    }

    fn load<T: DeserializeOwned>() -> Result<T, PersistError> {
        let load_bytes =
            std_fs::read(Self::config_path()?).map_err(|_| PersistError::Load(LoadError::Read))?;

        let loaded: T = serde_json::from_slice(&load_bytes)
            .map_err(|_| PersistError::Load(LoadError::Parse))?;

        Ok(loaded)
    }

    async fn _load_async<T: DeserializeOwned>() -> Result<T, PersistError> {
        let load_bytes = fs::read(Self::config_path()?)
            .await
            .map_err(|_| PersistError::Load(LoadError::Read))?;

        let loaded: T = serde_json::from_slice(&load_bytes)
            .map_err(|_| PersistError::Load(LoadError::Parse))?;

        Ok(loaded)
    }

    fn config_path() -> Result<PathBuf, PersistError>;
}

#[derive(Debug, Clone)]
pub enum PersistError {
    Save(SaveError),
    Load(LoadError),
    Path,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    Write,
    Compose,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    Read,
    Parse,
}
