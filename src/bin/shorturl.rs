use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;
use shorturl::{shorturl::ShortUrlLength, App, Config, Database};

/// `shorturl`: A short URL generator web service
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 7777)]
    port: u16,
    #[arg(short, long, default_value = "shorturl.db")]
    db_file: PathBuf,
    #[arg(short, long, default_value_t = 5)]
    short_url_length: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();

    if args.port == 0 {
        return Err(anyhow!("Port cannot be 0!"));
    }

    if args.short_url_length == 0 {
        return Err(anyhow!("Short URL length cannot be 0!"));
    }

    let db_file = args.db_file.as_path();
    let db_file_str = db_file.to_string_lossy();
    log::trace!("Loading database from {}", db_file_str);
    let db = Database::load(Some(db_file)).unwrap_or_default();
    log::trace!("Loaded database from {}", db_file_str);

    let mut config = Config::default();
    config.database = db;
    config.port = args.port;
    config.short_url_length = ShortUrlLength::new(args.short_url_length).unwrap();

    let _ = App::serve(config).await;

    Ok(())
}
