use jsonwebtoken::{encode, Header, EncodingKey};
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
}

/**
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        aud: String,         // Optional. Audience
        exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
        iat: usize,          // Optional. Issued at (as UTC timestamp)
        iss: String,         // Optional. Issuer
        nbf: usize,          // Optional. Not Before (as UTC timestamp)
        sub: String,         // Optional. Subject (whom token refers to)
    }
 */

pub fn jwt_factory(claims: Claims) -> String {
    let config = Config::from_env().expect("please set some env vars");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret_key.as_bytes()));
    match token {
        Ok(jwt) => jwt,
        _ => String::from("Could not create token")
    }
}