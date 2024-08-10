use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use axum::extract::State;
use axum::{routing::post, Json, Router};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use url::Url;

use crate::{
    database::Database,
    error::AppError,
    shorturl::{ShortUrl, ShortUrlLength},
};

pub struct App;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    base_url: Url,
    port: u16,
    short_url_length: ShortUrlLength,
    database: Database,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: Url::from_str("http://localhost:7777").unwrap(),
            port: 7777,
            short_url_length: ShortUrlLength::default(),
            database: Database::default(),
        }
    }
}

impl Config {
    pub fn set_database(&mut self, database: Database) {
        self.database = database;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ShortenParameters {
    url: Url,
}

type AppState = Arc<Mutex<Config>>;

impl App {
    async fn shorten(
        State(config): State<AppState>,
        Json(body): Json<ShortenParameters>,
    ) -> Result<String, AppError> {
        let mut config = config.lock().await;

        let short_path = ShortUrl::generate(&body.url, Some(config.short_url_length));
        let short_url = config.base_url.join(&short_path).map_err(|e| {
            log::error!("{}", &e);
            AppError::internal_error(e.to_string())
        })?;

        log::info!("{} => {}", &short_url, &body.url);
        if let Some(old_url) = config.database.set(short_path, body.url) {
            log::warn!(
                "Hashing collision for {} redirecting to {}",
                &short_url,
                old_url
            );
        }

        config.database.save()?;

        Ok(short_url.to_string())
    }

    pub fn serve(config: Config) -> tokio::task::JoinHandle<()> {
        let bind_address = format!("0.0.0.0:{}", config.port);
        let state = Arc::new(Mutex::new(config));
        let router = Router::new()
            .route("/:shorten", post(Self::shorten))
            .with_state(state);

        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
            info!("Serving at {}", bind_address);
            axum::serve(listener, router).await.unwrap();
            info!("Server exited for some reason!");
        })
    }
}
