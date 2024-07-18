use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn create_user(state: &web::Data<AppState>, new_user: &NewUser) -> Result<User, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    let new_user = NewUser {
        mail: new_user.mail.clone(),
        phone: new_user.phone.clone(),
        role: new_user.role.clone(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .map_err(|err| format!("Failed to insert user: {}", err))
}

pub async fn get_users(state: &web::Data<AppState>) -> Result<Vec<User>, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    users
        .load::<User>(&mut conn)
        .map_err(|err| format!("Failed to load users: {}", err))
}

pub async fn get_user(state: &web::Data<AppState>, user_id: i32) -> Result<User, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    users
        .filter(id.eq(user_id))
        .first::<User>(&mut conn)
        .map_err(|err| format!("User not found: {}", err))
}

pub async fn update_user(
    state: &web::Data<AppState>,
    user_id: i32,
    updated_user: &NewUser,
) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::update(users.find(user_id))
        .set((
            mail.eq(&updated_user.mail),
            phone.eq(&updated_user.phone),
            role.eq(&updated_user.role),
        ))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to update user: {}", err))
}

pub async fn delete_user(state: &web::Data<AppState>, user_id: i32) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::delete(users.find(user_id))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to delete user: {}", err))
}
