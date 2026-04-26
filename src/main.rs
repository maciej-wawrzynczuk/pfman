mod cache;
mod trans_log;
use crate::{cache::RedbCache, trans_log::TransLog};
use anyhow::Context;
use std::{env, fs::File};


fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("Starting up");

    let filename = env::args().nth(1).context("bad arguments")?;
    let f = File::open(filename)?;
    let tl = TransLog::from_reader(f)?;
    let j = serde_json::to_string_pretty(&tl)?;
    println!("{j}");

    _ = RedbCache::new();

    Ok(())
}
