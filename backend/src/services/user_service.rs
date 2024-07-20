use crate::models::database::DatabaseConnection;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_user(conn: &mut DatabaseConnection, new_user: &NewUser) -> Result<User, Error> {

    let new_user = NewUser {
        mail: new_user.mail.clone(),
        phone: new_user.phone.clone(),
        role: new_user.role.clone(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
}

pub async fn get_users(conn: &mut DatabaseConnection) -> Result<Vec<User>, Error> {
    users.load::<User>(conn)
}

pub async fn get_user(conn: &mut DatabaseConnection, user_id: i32) -> Result<User, Error> {
    users
        .filter(id.eq(user_id))
        .first::<User>(conn)
}

pub async fn update_user(
    conn: &mut DatabaseConnection,
    user_id: i32,
    updated_user: &NewUser,
) -> Result<User, Error> {
    diesel::update(users.find(user_id))
        .set((
            mail.eq(&updated_user.mail),
            phone.eq(&updated_user.phone),
            role.eq(&updated_user.role),
        ))
        .get_result(conn)
}

pub async fn delete_user(conn: &mut DatabaseConnection, user_id: i32) -> Result<User, Error> {
    diesel::delete(users.find(user_id))
        .get_result(conn)
}
