#[macro_use]
extern crate anyhow;
extern crate directories_next;
extern crate sled;
use sled::Db;
extern crate thiserror;
use thiserror::Error as _Error;
extern crate uuid;

use std::{
    path::Path,
    sync::Arc,
};
pub mod object;
pub mod library;


#[derive(_Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Db(#[from] sled::Error),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

pub struct System {
    pub db: Arc<Db>,
}
impl System {
    /// Synchronously flush the internal database to disk, returning the number of bytes flushed.
    pub fn flush(&self) -> Result<usize> {
        Ok(self.db.flush()?)
    }
    /// Asynchronously flush the internal database to disk, returning the number of bytes flushed.
    pub async fn flush_async(&self) -> Result<usize> {
        Ok(self.db.flush_async().await?)
    }
    /// Opens the database at the specified path.
    pub fn open_path(path: impl AsRef<Path>) -> Result<System> {
        let db = sled::open(path.as_ref())?;
        let db = Arc::new(db);
        Ok(System {
            db
        })
    }

    /// Opens the database in a sensible location for the host operating system. See https://docs.rs/directories-next/latest/directories_next/struct.ProjectDirs.html#method.data_local_dir
    pub fn open() -> Result<System> {
        if let Some(proj_dirs) = directories_next::ProjectDirs::from("com", "mcaveniathor", "rs400") {
            Self::open_path(proj_dirs.data_local_dir())
        }
        else {
            Err(Error::Other(anyhow!("Could not find local data directory, please manually specify a path.")))
        }
    }
}
