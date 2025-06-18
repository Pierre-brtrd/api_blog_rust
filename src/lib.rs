pub mod application {
    pub mod post_service;
    pub mod user_service;
}

pub mod config;

pub mod domain {
    pub mod model {
        pub mod post;
        pub mod user;
    }
    pub mod error;
    pub mod repository;
    pub mod validation;
}

pub mod infrastructure {
    pub mod auth;
    pub mod db;

    pub mod security {
        pub mod cors;
        pub mod hsts;
        pub mod keys;
        pub mod tls;
    }

    pub mod persistence {
        pub mod sqlite {
            pub mod post_repo;
            pub mod user_repo;
        }
    }
}

pub mod interfaces {
    pub mod api {
        pub mod error;
        pub mod validation;

        pub mod dto {
            pub mod post;
            pub mod user;
        }
        pub mod handlers {
            pub mod login;
            pub mod post;
            pub mod user;
        }

        pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
            handlers::user::config(cfg);
            handlers::post::config(cfg);
            handlers::login::config(cfg);
        }
    }
}
