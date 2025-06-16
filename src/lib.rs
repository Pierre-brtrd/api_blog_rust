pub mod core {
    pub mod db;
    pub mod server;
    pub mod tls;
}
pub mod api {
    pub mod error;
    pub mod handlers;
}

pub mod config {
    pub mod settings;
}

pub mod domain {
    pub mod error;
    pub mod post;
    pub mod repository;
}

pub mod infra {
    pub mod sqlite_post_repo;
}
