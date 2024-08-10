use std::{str::FromStr, sync::Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use log::{error, info};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::shorturl::{ShortUrl, ShortUrlLength};

pub struct App;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    base_url: Url,
    port: u16,
    short_url_length: ShortUrlLength,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: Url::from_str("http://localhost:7777").unwrap(),
            port: 7777,
            short_url_length: ShortUrlLength::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ShortenParameters {
    url: Url,
}

impl App {
    async fn shorten(
        State(config): State<Arc<Config>>,
        Json(body): Json<ShortenParameters>,
    ) -> impl IntoResponse {
        let short_url = ShortUrl::generate(&body.url, Some(config.as_ref().short_url_length));
        let short_url = config.as_ref().base_url.join(&short_url);

        short_url.map_or_else(
            |e| {
                error!("shorten(): {}", &e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            },
            |u| {
                info!("Shortend URL for {} is {}", body.url, &u);
                (StatusCode::OK, u.to_string())
            },
        )
    }

    pub fn serve(config: Config) -> tokio::task::JoinHandle<()> {
        let bind_address = format!("0.0.0.0:{}", config.port);
        let state = Arc::new(config);
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
