use crate::models::user::User;
use crate::models::water_closet::WaterCloset;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::States"]
pub enum States {
    #[db_rename = "TODO"]
    Todo,
    #[db_rename = "IN_PROGRESS"]
    InProgress,
    #[db_rename = "DONE"]
    Done,
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Topics"]
pub enum Topics {
    #[db_rename = "DOOR"]
    Door,
    #[db_rename = "TOILET"]
    Toilet,
    #[db_rename = "SUPPLY"]
    Supply,
    #[db_rename = "CLEANLINESS"]
    Cleanliness,
    #[db_rename = "OTHER"]
    Other,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(WaterCloset))]
#[diesel(table_name = crate::schema::reports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Report {
    pub id: i32,
    pub user_id: i32,
    pub water_closet_id: i32,
    pub datetime: NaiveDateTime,
    pub state: States,
    pub topic: Topics,
    pub comment: String,
}
