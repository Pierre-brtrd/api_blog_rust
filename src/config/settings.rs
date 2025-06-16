use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct TlsSettings {
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server: ServerSettings,
    pub tls: Option<TlsSettings>,
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
