mod trans_log;
use std::{env, fs::File};

use anyhow::Context;

use crate::trans_log::TransLog;

fn main() -> anyhow::Result<()> {
    let filename = env::args().nth(1).context("bad arguments")?;
    let f = File::open(filename)?;
    let tl = TransLog::from_reader(f)?;
    let j = serde_json::to_string_pretty(&tl)?;
    println!("{j}");

    Ok(())
}
