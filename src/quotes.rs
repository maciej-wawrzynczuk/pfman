use std::{fs::create_dir_all, path::{self, Path}};

use anyhow::Context;
use directories::ProjectDirs;
use redb::Database;

pub struct Quotes {
    db: Database,
}

impl Quotes {
    pub fn new() -> anyhow::Result<Self> {
        let proj_dir = ProjectDirs::from("com", "wawrzynczuk", "pfman")
            .context("Something nasty. Probably cannot determine homedir")?;
        let cache_dir = proj_dir.cache_dir();
        create_dir_all(cache_dir)?;
        log::info!("Using cache in {cache_dir:?}");

        let db_dir = cache_dir.join("db");
        let db = Database::create(&db_dir)?;
        log::info!("Using database {db_dir:?}");

        Ok(Self { db })
    }
}
