use anyhow::Result;
use shorturl::{App, Config, Database};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let db_file = "shorturl.db";
    log::trace!("Loading database from {}", db_file);
    let db = Database::load(Some(db_file)).unwrap_or_default();
    log::trace!("Loaded database from {}", db_file);

    let mut config = Config::default();
    config.set_database(db);

    let _ = App::serve(config).await;

    Ok(())
}
