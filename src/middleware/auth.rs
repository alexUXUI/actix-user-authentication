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

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

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

                let auth_header = req
                    .headers()
                    .get(actix_web::http::header::AUTHORIZATION);
                    
                if auth_header.unwrap().to_str().unwrap().is_empty() {

                    Box::pin(async { Err(ErrorUnauthorized("User not authorized")) })

                } else {

                    let config = Config::from_env().expect("Must set env vars in config file");
                    let validation = Validation { ..Validation::default() };
                
                    match decode::<Claims>(
                        &auth_header.unwrap().to_str().unwrap(), 
                        &DecodingKey::from_secret(config.jwt_secret_key.as_bytes()), 
                        &validation
                    ) {
                        Ok(token) => {

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
