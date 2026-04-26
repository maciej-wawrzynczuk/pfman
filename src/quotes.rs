use std::{fs::create_dir_all, path::{self, Path}};

use anyhow::Context;
use chrono::NaiveDate;
use directories::ProjectDirs;
use redb::{Database, TableDefinition};
use rust_decimal::Decimal;

pub struct Cache {
    db: Database,
}

const CACHE_TABLE_DEF: TableDefinition<&str, u128> = TableDefinition::new("cache");

impl Cache {
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
        fn key(symbol: &str, date: &NaiveDate) -> String {
            format!("{}:{}", symbol, date)
        }

    pub fn get(&self, _symbol: &str, _date: &NaiveDate) -> Option<Decimal> {
        None
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use super::*;

    #[test]
    fn test_key() {
        let d = NaiveDate::from_ymd_opt(2001, 01, 18).unwrap();
        let s = "foo";

        let k = Cache::key(s, &d);
        let expected = "foo:2001-01-18";
        assert_eq!(k, expected);
    }
}