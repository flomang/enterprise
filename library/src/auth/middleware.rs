use crate::db::Pool;
use actix_service::{Service, Transform};
use actix_web::body::EitherBody;

use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http,
    http::{header::HeaderName, header::HeaderValue, Method},
    web::Data,
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Authentication<'a> {
    secret: &'a [u8],
    ignore_routes: &'a [&'a str],
}

impl<'a> Authentication<'a> {
    pub fn new(secret: &'a[u8], ignore_routes: &'a[&'a str]) -> Authentication<'a> {
        Authentication { secret, ignore_routes }
    }
}

impl<'a, S, B> Transform<S, ServiceRequest> for Authentication<'a>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<'a, S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service,
            secret: self.secret,
            ignore_routes: self.ignore_routes,
        })
    }
}

pub struct AuthenticationMiddleware<'a, S> {
    service: S,
    secret: &'a [u8],
    ignore_routes: &'a [&'a str],
}

impl<'a, S, B> Service<ServiceRequest> for AuthenticationMiddleware<'a, S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        // Bypass some account routes
        let headers = req.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in self.ignore_routes.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
            if !authenticate_pass {
                if let Some(pool) = req.app_data::<Data<Pool>>() {
                    log::info!("Connecting to database...");
                    if let Some(authen_header) = req.headers().get("Authorization") {
                        log::info!("Parsing authorization header...");
                        if let Ok(authen_str) = authen_header.to_str() {
                            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer")
                            {
                                log::info!("Parsing token...");
                                let token = authen_str[6..authen_str.len()].trim();
                                if let Ok(token_data) =
                                    super::decode_token(token.to_string(), self.secret)
                                {
                                    log::info!("Decoding token...");
                                    if super::verify_token(&token_data, pool).is_ok() {
                                        log::info!("Valid token");
                                        authenticate_pass = true;
                                    } else {
                                        log::error!("Invalid token");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Don't forward to `/login` if we are already on `/login`.
        if !authenticate_pass && req.path() != "api/login" {
            let (request, _pl) = req.into_parts();

            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "api/login"))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(req);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
