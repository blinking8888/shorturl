use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::routing::get;
use axum::{routing::post, Json, Router};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use url::Url;
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::shorturl::ShortPath;
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

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
struct ShortenParameters {
    #[param(style=Form, example = "https://www.example.com", required, allow_reserved)]
    url: Url,
}

type AppState = Arc<Mutex<Config>>;

#[derive(OpenApi)]
#[openapi(paths(shorten), components(schemas(ShortenParameters)),    tags(
    (name = "shorten", description = "Simple URL Shortener")
),)]
pub struct ShortenUrlApi;

/// Shortens the given URL
#[utoipa::path(
    tag = "shorten",
    post,
    path = "/shorten",
    params(ShortenParameters),
    responses(
        (status = 200, description = "Shortened the given URL", body = String),
        (status = 500, description = "The server encountered an error", body = String)
    )
)]
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

impl App {
    async fn short_path(
        Path(short_path): Path<ShortPath>,
        State(config): State<AppState>,
    ) -> Result<Response<Body>, AppError> {
        let db = &config.lock().await.database;
        if let Some(redirect_url) = db.get(&short_path) {
            info!("Redirecting {} => {}", short_path.as_str(), &redirect_url);
            Response::builder()
                .header("Location", redirect_url.to_string())
                .status(StatusCode::FOUND)
                .body(Body::empty())
                .map_err(|e| AppError::internal_error(e.to_string()))
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .map_err(|e| AppError::internal_error(e.to_string()))
        }
    }

    pub fn serve(config: Config) -> tokio::task::JoinHandle<()> {
        let bind_address = format!("0.0.0.0:{}", config.port);
        let state = Arc::new(Mutex::new(config));
        let router = Router::new()
            .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ShortenUrlApi::openapi()))
            .route("/shorten", post(shorten))
            .route("/:short_path", get(Self::short_path))
            .with_state(state);

        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
            info!("Serving at {}", bind_address);
            axum::serve(listener, router).await.unwrap();
            info!("Server exited for some reason!");
        })
    }
}
