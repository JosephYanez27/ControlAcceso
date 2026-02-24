use jsonwebtoken::{decode, DecodingKey, Validation};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use crate::models::claims::Claims;

/* =========================
   STRUCT PRINCIPAL
========================= */
pub struct JwtMiddleware;

/* =========================
   VALIDAR TOKEN
========================= */
pub fn validate_token(token: &str) -> Result<Claims, Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map_err(|_| actix_web::error::ErrorUnauthorized("Token inválido"))?;

    Ok(token_data.claims)
}

/* =========================
   TRANSFORM
========================= */
impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareMiddleware { service })
    }
}

/* =========================
   MIDDLEWARE INTERNO
========================= */
pub struct JwtMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self,  req: ServiceRequest) -> Self::Future {

        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
            if let Ok(auth_str) = header_value.to_str() {
                if auth_str.starts_with("Bearer ") {

                    let token = auth_str.trim_start_matches("Bearer ");

                    if let Ok(claims) = validate_token(token) {
                        req.extensions_mut().insert(claims);

                        let fut = self.service.call(req);
                        return Box::pin(async move { fut.await });
                    }
                }
            }
        }

        // 🔥 SOLUCIÓN CORRECTA SIN EitherBody
        Box::pin(async move {
            Err(actix_web::error::ErrorUnauthorized("No autorizado"))
        })
    }
}