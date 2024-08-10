use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use url::Url;

use crate::shorturl::ShortUrl;

pub struct Database {
    file: PathBuf,
    data: HashMap<ShortUrl, Url>,
}

impl Database {
    pub fn load<P: Into<PathBuf>>(file: P) -> Result<Self> {
        let file: PathBuf = file.into();
        let mut f = File::open(file.as_path())?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let data: HashMap<ShortUrl, Url> = toml::from_str(buf.as_str())?;
        Ok(Self { file, data })
    }

    pub fn save(&self) -> Result<()> {
        let mut f = File::open(self.file.as_os_str())?;
        let data = toml::to_string_pretty(&self.data)?;
        f.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn get(&self, short_url: &ShortUrl) -> Option<&Url> {
        self.data.get(short_url)
    }

    pub fn set(&mut self, short_url: ShortUrl, long_url: Url) -> Result<Url> {
        self.data
            .insert(short_url.clone(), long_url)
            .ok_or(anyhow!("Cannot create short URL: {}.", short_url))
    }
}
