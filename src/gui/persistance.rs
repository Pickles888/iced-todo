use std::{fs as std_fs, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};
use tokio::fs;

pub trait Persistance {
    async fn save<T: Serialize>(items: &T) -> Result<(), PersistError> {
        let save_string =
            serde_json::to_string(&items).map_err(|_| PersistError::Save(SaveError::Compose))?;

        fs::write(Self::path()?, save_string)
            .await
            .map_err(|_| PersistError::Save(SaveError::Write))?;

        Ok(())
    }

    fn load<T: DeserializeOwned>() -> Result<T, PersistError> {
        let load_bytes =
            std_fs::read(Self::path()?).map_err(|_| PersistError::Load(LoadError::Read))?;

        let loaded: T = serde_json::from_slice(&load_bytes)
            .map_err(|_| PersistError::Load(LoadError::Parse))?;

        Ok(loaded)
    }

    async fn load_async<T: DeserializeOwned>() -> Result<T, PersistError> {
        let load_bytes = fs::read(Self::path()?)
            .await
            .map_err(|_| PersistError::Load(LoadError::Read))?;

        let loaded: T = serde_json::from_slice(&load_bytes)
            .map_err(|_| PersistError::Load(LoadError::Parse))?;

        Ok(loaded)
    }

    fn path() -> Result<PathBuf, PersistError>;
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
