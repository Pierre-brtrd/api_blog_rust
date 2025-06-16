use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use anyhow::Result;
use api_back_trio::api::handlers::config;
use api_back_trio::config::settings::Settings;
use api_back_trio::core::db::init_db;
use api_back_trio::core::server::AppState;
use api_back_trio::core::tls::build_ssl_acceptor;
use api_back_trio::infra::sqlite_post_repo::SqlitePostRepo;
use env_logger::Env;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default()).init();

    let settings = Settings::from_env()?;
    let pool = init_db(&settings.database_url).await?;
    let repo = SqlitePostRepo::new(pool.clone());
    let state = AppState::new(repo);
    let ssl = build_ssl_acceptor(
        settings.tls.as_ref().unwrap().cert_path.as_str(),
        settings.tls.as_ref().unwrap().key_path.as_str(),
    )?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .configure(config)
    })
    .bind_openssl((settings.server.host.as_str(), settings.server.port), ssl)?
    .run()
    .await?;

    Ok(())
}
