use crate::models::auth::Claims;
use crate::service::auth_service::validate_jwt;
use actix_web::FromRequest;
use actix_web::body::EitherBody;
use actix_web::{
    Error, HttpMessage, HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::FutureExt;
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::task::{Context, Poll};
use tracing::warn;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static + Clone,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner { service })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static + Clone,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let (http_req, mut payload) = req.parts_mut();

        let fut = BearerAuth::from_request(http_req, &mut payload);
        let srv = self.service.clone();

        async move {
            let bearer = match fut.await {
                Ok(bearer) => bearer,
                Err(_) => {
                    warn!("⛔️ No Bearer token provided");
                    let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                    return Ok(req.into_response(res));
                }
            };

            let claims: Claims = match validate_jwt(bearer.token()) {
                Ok(c) => c,
                Err(_) => {
                    warn!("⚠️ Invalid JWT: {}", bearer.token());
                    let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                    return Ok(req.into_response(res));
                }
            };

            req.extensions_mut().insert(claims);
            let res = srv.call(req).await?;
            Ok(res.map_into_left_body())
        }
        .boxed_local()
    }
}
