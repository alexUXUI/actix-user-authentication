use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, error::ErrorUnauthorized};

use futures::future::{ok, Ready};
use futures::Future;

use crate::modules::jwt::{validate_token};

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

        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION);
            
        match auth_header {
            Some(access_token) => {

                let token_is_still_valid = validate_token(
                    &access_token.to_str().unwrap().to_string()
                );
            
                match token_is_still_valid {
                    true => {
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    },
                    false => {
                        Box::pin(async { 
                            Err(ErrorUnauthorized("JWT invalid")) 
                        })
                    }
                }

            },
            None => {
                Box::pin(async { 
                    Err(ErrorUnauthorized("No Authorization header on request")) 
                })
            }
        }
    }
}
