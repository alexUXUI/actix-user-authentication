use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, error::ErrorUnauthorized};

use jsonwebtoken::{decode, Validation, DecodingKey};
use futures::future::{ok, Ready};
use futures::Future;

use crate::config::Config;
use crate::modules::jwt::{Claims};

#[derive(Debug)]
pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

#[derive(Debug)]
pub struct AuthMiddleware<S> {
    service: S,
}

// Trait actix_service::Service 
// An asynchronous operation from Request to a Response.
// https://docs.rs/actix-service/1.0.6/actix_service/trait.Service.html
impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    
    // To poll a future it has to be pinned to memory so that it will never move.
    // Pinning a box pins it to the heap which is nice bc unlike stack pinning, 
    // we know that the data will be pinned for the lifetime of the object.
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    // Returns Ready when the service is able to process requests.
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    // Process the request and return the response asynchronously.
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let login_route = String::from("/app/login");

        match req.path() == login_route {
            true => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            false => {

                let auth_header: &str = req
                    .headers()
                    .get(actix_web::http::header::AUTHORIZATION)
                    .unwrap()
                    .to_str()
                    .unwrap();
                    
                if auth_header.is_empty() {

                    Box::pin(async { Err(ErrorUnauthorized("User not authorized")) })

                } else {

                    let config = Config::from_env().expect("Must set env vars in config file");
                    let validation = Validation { ..Validation::default() };
                
                    match decode::<Claims>(
                        &auth_header, 
                        &DecodingKey::from_secret(config.jwt_secret_key.as_bytes()), 
                        &validation
                    ) {
                        Ok(_) => {
                            let fut = self.service.call(req);
                            Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            })
                        },
                        Err(_) => {
                            Box::pin(async { Err(ErrorUnauthorized("JWT invalid")) })
                        }
                    }
                }
            }
        }
    }
}
