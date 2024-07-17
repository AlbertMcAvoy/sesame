use crate::models::group::Group;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::CleanStates"]
pub enum CleanStates {
    #[db_rename = "CLEANED"]
    Cleaned,
    #[db_rename = "USED"]
    Used,
    #[db_rename = "DIRTY"]
    Dirty,
    #[db_rename = "OUT_OF_ORDER"]
    OutOfOrder,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(belongs_to(Group))]
#[diesel(table_name = crate::schema::water_closets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WaterCloset {
    pub id: i32,
    pub group_id: i32,
    pub is_disabled: bool,
    pub is_available: bool,
    pub is_door_opened: bool,
    pub is_door_locked: bool,
    pub clean_state: CleanStates,
}
