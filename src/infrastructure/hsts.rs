use actix_web::{
    Error,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
};
use futures_util::future::{LocalBoxFuture, Ready, ready};

pub struct Hsts;

impl<S, B> Transform<S, ServiceRequest> for Hsts
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = HstsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HstsMiddleware { service }))
    }
}

pub struct HstsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for HstsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?; // <-- await à l’intérieur d’un async move, c’est OK
            res.response_mut().headers_mut().insert(
                header::STRICT_TRANSPORT_SECURITY,
                header::HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
            );
            // on convertit le body original B en EitherBody<B> pour respecter le type
            Ok(res.map_into_left_body())
        })
    }
}
