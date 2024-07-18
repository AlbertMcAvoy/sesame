use crate::models::user::User;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}

#[derive(Insertable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::groups)]
pub struct NewGroup {
    pub user_id: i32,
    pub name: String,
}
