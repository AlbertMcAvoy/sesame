use crate::models::database::DatabaseConnection;
use crate::models::user::{NewUser, Roles, User};
use crate::schema::users::dsl::*;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthError {
    pub msg: String
}

pub async fn authentificate(
    conn: &mut DatabaseConnection,
    mail_input: &str,
) -> Result<String, AuthError> {

    match users.filter(mail.eq(mail_input)).first::<User>(conn) {
        Ok(user) => match generate_token(user) {
            Ok(token) => Ok(token),
            Err(err) => Err(AuthError {msg: format!("{:?}", err)})
        },
        Err(_) => {
            // Create a new user if not found
            let new_user = NewUser {
                mail: mail_input.to_string(),
                phone: None,
                role: Roles::User,
            };
            match diesel::insert_into(users).values(&new_user).get_result(conn) {
                Ok(user) => match generate_token(user) {
                    Ok(token) => Ok(token),
                    Err(err) => Err(AuthError {msg: format!("{:?}", err)})
                },
                Err(err) => Err(AuthError {msg: format!("{:?}", err)})
            }
        }
    }
}

fn generate_token(user: User) -> Result<String, JWTError> {
    // Generate JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.mail.clone(),
        exp: expiration,
    };

    let secret_key = env::var("JWT_SECRET_KEY").expect("SECRET_KEY must be set");

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

pub fn get_sub_from_token(token: &str) -> Result<String, AuthError> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("SECRET_KEY must be set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(err) => Err(AuthError {msg: format!("{:?}", err)}),
    }
}
