use jsonwebtoken::{encode, Header, EncodingKey};
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
}

pub fn jwt_factory() -> String {
    let config = Config::from_env().expect("please set some env vars");
    let claims = Claims { exp: 10000000000 };
    match encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret_key.as_bytes())) {
        Ok(token) => token,
        _ => String::from("Could not create token")
    }
}