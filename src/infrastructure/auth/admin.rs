use std::{
    sync::Arc,
    task::{Context, Poll},
};

use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    web,
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use jsonwebtoken::{Validation, decode};
use serde_json::json;

use crate::{
    domain::model::user::Role,
    infrastructure::{auth::Claims, keys::Keys},
};

pub struct AdminMiddleware;

impl AdminMiddleware {
    pub fn new() -> Self {
        AdminMiddleware
    }
}

impl Default for AdminMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AdminMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminMiddlewareService {
            service: Arc::new(service),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AdminMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        let keys_opt = req.app_data::<web::Data<Keys>>().cloned();
        let token_opt = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer ").map(str::to_owned));

        Box::pin(async move {
            let claims = if let (Some(keys), Some(token)) = (keys_opt, token_opt) {
                match decode::<Claims>(&token, &keys.decoding, &Validation::default()) {
                    Ok(data) => data.claims,
                    Err(_) => {
                        let body = json!({
                            "error": "Invalid token"
                        });

                        let resp = HttpResponse::Unauthorized().json(body);

                        return Ok(req.into_response(resp));
                    }
                }
            } else {
                let body = json!({ "error": "Missing Authorization header" });
                let resp = HttpResponse::Unauthorized().json(body);
                return Ok(req.into_response(resp));
            };

            if claims.role != Role::Admin {
                let body = json!({
                    "error": "Forbidden: Admin access required"
                });
                let resp = HttpResponse::Forbidden().json(body);
                return Ok(req.into_response(resp));
            }

            let res = srv.call(req).await?;

            Ok(res.map_into_boxed_body())
        })
    }
}
