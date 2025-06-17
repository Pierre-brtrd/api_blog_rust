use futures_util::future::{LocalBoxFuture, Ready, ready};
use jsonwebtoken::{Validation, decode};
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

use crate::core::{auth::Claims, keys::Keys};

pub struct JwtMiddleware;

impl JwtMiddleware {
    pub fn new() -> Self {
        JwtMiddleware
    }
}

pub struct JwtMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
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
        // Clone keys
        let keys_opt = req.app_data::<web::Data<Keys>>().cloned();

        // Extract token
        let token_opt = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer ").map(str::to_owned));

        // clone service for async call
        let srv = self.service.clone();

        Box::pin(async move {
            let valid = if let (Some(keys), Some(token)) = (keys_opt, token_opt) {
                decode::<Claims>(&token, &keys.decoding, &Validation::default()).is_ok()
            } else {
                false
            };

            if !valid {
                let body = serde_json::json!({"error": "Invalid token"});
                let resp = HttpResponse::Unauthorized().json(body);
                return Ok(req.into_response(resp));
            }

            // on utilise le clone, plus de borrow de self
            let res = srv.call(req).await?;
            Ok(res.map_into_boxed_body())
        })
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service: Arc::new(service),
        }))
    }
}
