use std::sync::Arc;
use url_wrap_driver::module::Modules;
use url_wrap_driver::startup::{init_app, startup};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    let _ = startup(Arc::new(modules)).await;

    Ok(())
}
