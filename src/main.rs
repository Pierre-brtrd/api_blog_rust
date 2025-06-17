use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use anyhow::Result;
use api_back_trio::api::config as api_config;
use api_back_trio::config::settings::Settings;
use api_back_trio::core::db::init_db;
use api_back_trio::core::keys::Keys;
use api_back_trio::core::tls::build_ssl_acceptor;
use api_back_trio::infra::sqlite_post_repo::SqlitePostRepo;
use api_back_trio::infra::sqlite_user_repo::SqliteUserRepo;
use env_logger::Env;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default()).init();

    let settings = Settings::from_env()?;
    let pool = init_db(&settings.database_url).await?;
    let post_repo = SqlitePostRepo::new(pool.clone());
    let user_repo = SqliteUserRepo::new(pool.clone());
    let keys = Keys::new(settings.jwt_secret.as_bytes());

    let ssl = build_ssl_acceptor(
        settings.tls.as_ref().unwrap().cert_path.as_str(),
        settings.tls.as_ref().unwrap().key_path.as_str(),
    )?;

    let server_settings = settings.server.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(post_repo.clone()))
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(keys.clone()))
            .app_data(web::Data::new(settings.clone()))
            .configure(api_config)
    })
    .bind_openssl((server_settings.host.as_str(), server_settings.port), ssl)?
    .run()
    .await?;

    Ok(())
}
