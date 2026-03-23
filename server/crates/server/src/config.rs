pub struct GameServerConfig {
    pub addr: String,
}

impl GameServerConfig {
    pub fn load() -> Self {
        // @TODO load config from .env depending on dev/prod
        dotenvy::dotenv().ok();
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

        GameServerConfig {
            addr: format!("{host}:{port}"),
        }
    }
}
