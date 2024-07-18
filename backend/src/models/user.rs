use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Roles"]
pub enum Roles {
    #[db_rename = "USER"]
    User,
    #[db_rename = "CLIENT"]
    Client,
    #[db_rename = "WORKER"]
    Worker,
    #[db_rename = "ADMIN"]
    Admin,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub mail: String,
    pub phone: Option<String>,
    pub role: Roles,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub mail: String,
    pub phone: Option<String>,
    pub role: Roles,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub mail: String,
    pub phone: String,
    pub role: Roles,
}
