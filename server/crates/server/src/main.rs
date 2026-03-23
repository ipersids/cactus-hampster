use tokio::net::TcpListener;

use server::config::GameServerConfig;
use server::router::GameServerRouter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GameServerConfig::load();

    let app = GameServerRouter::init();

    let listener = TcpListener::bind(config.addr).await?;
    axum::serve(listener, app.service).await?;

    Ok(())
}
