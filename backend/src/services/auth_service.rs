use crate::models::user::{NewUser, Roles, User};
use crate::schema::users::dsl::*;
use crate::AppState;
use actix_web::web;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn authentificate(
    state: &web::Data<AppState>,
    mail_input: &str,
) -> Result<String, String> {
    let mut conn = AppState::get_conn(&state);

    let user_result = users
        .filter(mail.eq(mail_input))
        .first::<User>(&mut conn)
        .optional();

    let user = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Create a new user if not found
            let new_user = NewUser {
                mail: mail_input.to_string(),
                phone: None,
                role: Roles::User,
            };
            diesel::insert_into(users)
                .values(&new_user)
                .get_result(&mut conn)
                .map_err(|err| format!("Error creating new user: {}", err))?
        }
        Err(err) => {
            return Err(format!("Database error: {}", err));
        }
    };

    // Generate JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.mail.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()),
    )
    .map_err(|err| format!("Error generating JWT: {}", err))?;

    Ok(token)
}
