use actix_web::middleware::errhandlers::{ ErrorHandlerResponse};
use actix_web::{dev, http, Error};

pub fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    res.response_mut()
       .headers_mut()
       .insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("Error"));
    Ok(ErrorHandlerResponse::Response(res))
}