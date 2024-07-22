use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::user::User;

use super::errors::business_errors::AuthError;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_token(user: User) -> Result<String, Error> {
    // Generate JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.mail.clone(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_secret_key().as_ref()))
}

pub fn get_sub_from_token(token: &str) -> Result<String, AuthError> {
    let decoding_key = DecodingKey::from_secret(get_secret_key().as_ref());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(err) => Err(AuthError {msg: format!("{:?}", err)}),
    }
}

fn get_secret_key() -> String {
    env::var("JWT_SECRET_KEY").expect("SECRET_KEY must be set")
}