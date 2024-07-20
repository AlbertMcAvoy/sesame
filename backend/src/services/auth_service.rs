use crate::helpers::auth_helper::{generate_token, get_sub_from_token, AuthError};
use crate::models::database::DatabaseConnection;
use crate::models::user::{NewUser, Roles, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;

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

pub fn get_mail_from_token(token: &str) -> Result<String, AuthError> {
    get_sub_from_token(token)
}