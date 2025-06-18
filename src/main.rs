use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use anyhow::Result;
use api_back_trio::application::post_service::PostService;
use api_back_trio::application::user_service::UserService;
use api_back_trio::config::Settings;
use api_back_trio::infrastructure::security::tls::build_ssl_acceptor;
use api_back_trio::infrastructure::{
    db::init_db,
    persistence::sqlite::{post_repo::SqlitePostRepo, user_repo::SqliteUserRepo},
    security::cors::build_cors,
    security::hsts::Hsts,
    security::keys::Keys,
};
use api_back_trio::interfaces::api::config as api_config;
use env_logger::Env;

#[actix_web::main]
async fn main() -> Result<()> {
    let settings = Settings::from_env()?;

    env_logger::Builder::from_env(Env::default()).init();
    let pool = init_db(&settings.database_url).await?;
    let post_repo = SqlitePostRepo::new(pool.clone());
    let user_repo = SqliteUserRepo::new(pool.clone());
    let keys = Keys::new(settings.jwt_secret.as_bytes());
    let post_service = PostService::new(post_repo, user_repo.clone());
    let user_service = UserService::new(user_repo, keys.clone());
    let ssl = build_ssl_acceptor(
        &settings.tls.as_ref().unwrap().cert_path,
        &settings.tls.as_ref().unwrap().key_path,
    )?;
    let server_settings = settings.server.clone();
    HttpServer::new(move || {
        let cors_middleware: Cors = build_cors(&settings.cors_origin);

        App::new()
            .wrap(Hsts)
            .wrap(cors_middleware)
            .wrap(Logger::default())
            .app_data(web::Data::new(post_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(keys.clone()))
            .app_data(web::Data::new(settings.clone()))
            .configure(api_config)
    })
    .bind_openssl((server_settings.host, server_settings.port), ssl)?
    .run()
    .await?;

    Ok(())
}
