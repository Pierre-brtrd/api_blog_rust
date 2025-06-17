use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct TlsSettings {
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub server: ServerSettings,
    pub tls: Option<TlsSettings>,
    pub jwt_secret: String,
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        let s = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}
