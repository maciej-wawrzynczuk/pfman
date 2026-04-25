mod trans_log;
mod quotes;
use std::{env, fs::File};
use anyhow::Context;
use crate::{quotes::Quotes, trans_log::TransLog};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("Starting up");

    let filename = env::args().nth(1).context("bad arguments")?;
    let f = File::open(filename)?;
    let tl = TransLog::from_reader(f)?;
    let j = serde_json::to_string_pretty(&tl)?;
    println!("{j}");

    _ = Quotes::new();

    Ok(())
}
