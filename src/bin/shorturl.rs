use anyhow::Result;
use shorturl::{App, Config};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let config = Config::default();
    let _ = App::serve(config).await;

    Ok(())
}
