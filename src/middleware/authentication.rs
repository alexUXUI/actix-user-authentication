use actix_web_httpauth::extractors::basic::{BasicAuth};
use actix_web::{dev::ServiceRequest, Error};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

/**
 * put this in the middleware
    let validation = Validation { sub: Some("b@b.com".to_string()), ..Validation::default() };
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };
*/

pub async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth, // @todo change this to bearer scheme
) -> Result<ServiceRequest, Error> {
    println!("request {:#?}", req);
    Ok(req)
}
