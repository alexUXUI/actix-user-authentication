use actix_web_httpauth::extractors::basic::{BasicAuth};
use actix_web::{dev::ServiceRequest, Error};

pub async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth, // @todo change this to bearer scheme
) -> Result<ServiceRequest, Error> {
    println!("request {:#?}", req);
    Ok(req)
}
