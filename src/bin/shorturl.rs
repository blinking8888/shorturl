use anyhow::Result;
use shorturl::{App, Config, Database};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let db = Database::load(Some("./shorturl.db")).unwrap_or_default();
    log::trace!("Database: {:#?}", &db);
    let mut config = Config::default();
    config.set_database(db);
    let _ = App::serve(config).await;

    Ok(())
}
