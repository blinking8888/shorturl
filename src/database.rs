use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use anyhow::Result;
use log::trace;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::shorturl::ShortPath;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Database {
    file: Option<PathBuf>,
    data: HashMap<ShortPath, Url>,
}

impl Database {
    pub fn load<P: Into<PathBuf>>(file: Option<P>) -> Result<Self> {
        if let Some(file) = file {
            let file: PathBuf = file.into();

            let mut f = File::open(file.as_path()).or_else(|e| {
                log::warn!("Open file error<{:#?}>: {}", file.to_str(), e);
                File::create_new(file.as_path())
            })?;
            let mut buf = String::new();

            f.read_to_string(&mut buf)?;

            let data: HashMap<ShortPath, Url> = toml::from_str(buf.as_str()).unwrap_or_default();

            Ok(Self {
                file: Some(file),
                data,
            })
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        if let Some(ref file) = self.file {
            let data = toml::to_string_pretty(&self.data)?;
            fs::write(file.as_path(), data.as_bytes())?;
            log::trace!("Saved!");
        }
        Ok(())
    }

    pub fn get(&self, short_path: &ShortPath) -> Option<&Url> {
        self.data.get(short_path)
    }

    pub fn set(&mut self, short_path: ShortPath, long_url: Url) -> Option<Url> {
        trace!("{} => {long_url}", short_path.as_str());
        self.data.insert(short_path, long_url)
    }
}
