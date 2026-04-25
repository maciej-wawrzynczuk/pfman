use anyhow::Context;
use directories::ProjectDirs;

pub struct Quotes {}

impl Quotes {
    pub fn new() -> anyhow::Result<Self> {
        let proj_dir = ProjectDirs::from(
            "com",
            "wawrzynczuk",
            "pfman"
        )
            .context("Something nasty. Probably cannot determine homedir")?;
        let cache_dir = proj_dir.cache_dir();
        log::info!("Using cache in {cache_dir:?}");
        Ok(Self{})
    }
}