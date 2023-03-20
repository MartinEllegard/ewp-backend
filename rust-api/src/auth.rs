use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // The subject (user id)
    pub exp: usize,  // Expiration time
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self {
        Claims { sub, exp }
    }

    pub fn encode(&self, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
        let encoding_key = EncodingKey::from_secret(secret);
        encode(&Header::default(), self, &encoding_key)
    }

    pub fn decode(token: &str, secret: &[u8]) ->
    Result<Self, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret);
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
        }
}

pub struct Jwt(pub String);

impl FromRequest for Jwt {
    // type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer") {
                    let token = auth_str[7..].trim();
                    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                    if let Ok(claims) = Claims::decode(token, secret.as_bytes()) {
                        return ready(Ok(Jwt(claims.sub)));
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
                    }
    }