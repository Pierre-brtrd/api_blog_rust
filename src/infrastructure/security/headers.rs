use actix_web::middleware::DefaultHeaders;

pub fn secure_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
        .add(("Content-Security-Policy", "default-src 'none';"))
        .add(("Referrer-Policy", "no-referrer"))
        .add((
            "Permissions-Policy",
            "geolocation=(), microphone=(), camera=(), interest-cohort=()",
        ))
        .add(("X-Permitted-Cross-Domain-Policies", "none"))
}
