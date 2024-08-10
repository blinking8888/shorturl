use std::{fmt::Display, ops::Deref};

use anyhow::{anyhow, Result};
use base_62::base62;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub struct ShortUrl(Url);

impl From<Url> for ShortUrl {
    fn from(value: Url) -> Self {
        Self(value)
    }
}

pub const DEFAULT_SHORTENED_LENGTH: u8 = 5;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(transparent)]
pub struct ShortUrlLength(u8);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[serde(transparent)]
pub struct ShortPath(String);

impl ShortPath {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for ShortPath {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl ShortUrlLength {
    pub fn new(length: u8) -> Result<Self> {
        if length > 0 {
            Ok(Self(length))
        } else {
            Err(anyhow!("ShortUrlLength cannot be zero!"))
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Default for ShortUrlLength {
    fn default() -> Self {
        Self::new(DEFAULT_SHORTENED_LENGTH).unwrap()
    }
}

impl Deref for ShortUrl {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl Display for ShortUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl ShortUrl {
    pub fn generate(long_url: &Url, short_length: Option<ShortUrlLength>) -> ShortPath {
        let mut hash = base62::encode(long_url.as_str().as_bytes());
        hash.truncate(short_length.unwrap_or_default().value().into());
        ShortPath(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_url_length_default() {
        assert_eq!(ShortUrlLength::default().value(), DEFAULT_SHORTENED_LENGTH);
    }
}
