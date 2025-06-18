use std::sync::Arc;

use actix_cors::Cors;
use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE, HeaderName};
use regex::Regex;

pub fn build_cors(origins: &str) -> Cors {
    let regexes: Vec<Regex> = origins
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|pat| {
            Regex::new(pat)
                .unwrap_or_else(|e| panic!("Invalid CORS regex pattern '{}': {}", pat, e))
        })
        .collect();

    let regexes = Arc::new(regexes);

    Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            AUTHORIZATION,
            CONTENT_TYPE,
            HeaderName::from_static("x-requested-with"),
        ])
        .max_age(21_600)
        .allowed_origin_fn(move |origin, _req_head| {
            origin
                .to_str()
                .map(|s| regexes.iter().any(|re| re.is_match(s)))
                .unwrap_or(false)
        })
        .supports_credentials()
}
