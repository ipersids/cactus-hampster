use tokio::net::TcpListener;

use server::config::GameServerConfig;
use server::router::GameServerRouter;
use server::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GameServerConfig::load();
    let state = AppState::new();

    println!("Starting server on {}", config.addr);
    let app = GameServerRouter::init(state);

    let listener = TcpListener::bind(config.addr).await?;
    axum::serve(listener, app.service).await?;

    Ok(())
}
