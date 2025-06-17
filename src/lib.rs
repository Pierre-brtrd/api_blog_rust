pub mod core {
    pub mod auth;
    pub mod db;
    pub mod jwt_middleware;
    pub mod keys;
    pub mod tls;
}
pub mod api {
    pub mod error;
    pub mod login;
    pub mod post;
    pub mod user;

    pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
        user::config(cfg);
        post::config(cfg);
        login::config(cfg);
    }
}

pub mod config {
    pub mod settings;
}

pub mod domain {
    pub mod error;
    pub mod post;
    pub mod repository;
    pub mod user;
    pub mod validation;
}

pub mod infra {
    pub mod sqlite_post_repo;
    pub mod sqlite_user_repo;
}
