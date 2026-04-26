use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::Context;
use chrono::NaiveDate;
use directories::ProjectDirs;
use redb::{Database, ReadableDatabase, TableDefinition};
use rust_decimal::Decimal;

pub struct Cache {
    db: Database,
}

const CACHE_TABLE_DEF: TableDefinition<&str, [u8; 16]> = TableDefinition::new("cache");

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        let db_file = Cache::default_dir()?;
        Cache::new_with_in(&db_file)
    }
    pub fn new_with_in(cache_dir: &Path) -> anyhow::Result<Self> {
        let db_file = cache_dir.join("db");
        let db = Database::create(&db_file)?;
        log::info!("Using database {db_file:?}");

        Ok(Self { db })
    }

    fn default_dir() -> anyhow::Result<PathBuf> {
        let proj_dir = ProjectDirs::from("com", "wawrzynczuk", "pfman")
            .context("Something nasty. Probably cannot determine homedir")?;
        let cache_dir = proj_dir.cache_dir();
        create_dir_all(cache_dir)?;
        Ok(cache_dir.to_path_buf())
    }

    fn key(symbol: &str, date: NaiveDate) -> String {
        format!("{}:{}", symbol, date)
    }

    pub fn get(&self, symbol: &str, date: NaiveDate) -> anyhow::Result<Option<Decimal>> {
        let txn = self.db.begin_read()?;
        let t = txn.open_table(CACHE_TABLE_DEF)?;
        let k = Cache::key(symbol, date);
        let v = t.get(&k.as_str())?;
        Ok(v.map(|g| {
            Decimal::deserialize(g.value())
        }))
    }

    pub fn set(&self, symbol: &str, date: NaiveDate, quote: Decimal) -> anyhow::Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut t = txn.open_table(CACHE_TABLE_DEF)?;
            let k = Cache::key(symbol, date);
            let i = quote.serialize();
            t.insert(k.as_str(), i)?;
        }
        txn.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::dec;
    use tempfile::tempdir;

    #[test]
    fn test_key() {
        let d = NaiveDate::from_ymd_opt(2001, 1, 18).unwrap();
        let s = "foo";

        let k = Cache::key(s, d);
        let expected = "foo:2001-01-18";
        assert_eq!(k, expected);
    }

    #[test]
    fn cache_roundtrip() {
        let tmp = tempdir().unwrap();
        let sut = Cache::new_with_in(tmp.path()).unwrap();
        let d = NaiveDate::from_ymd_opt(2001, 1, 18).unwrap();
        let s = "foo";
        let q = dec!(10.8);

        sut.set(s, d, q).unwrap();
        assert_eq!(sut.get(s, d).unwrap().unwrap(), q);
    }
}
