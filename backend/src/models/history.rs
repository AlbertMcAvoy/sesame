use crate::models::water_closet::WaterCloset;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[ExistingTypePath = "crate::schema::sql_types::Actions"]
pub enum Actions {
    #[db_rename = "DOOR_OPENING"]
    DoorOpening,
    #[db_rename = "DOOR_CLOSING"]
    DoorClosing,
    #[db_rename = "LOCK_OPENING"]
    LockOpening,
    #[db_rename = "LOCK_CLOSING"]
    LockClosing,
    #[db_rename = "QR_CODE_SCAN"]
    QRCodeScan,
    #[db_rename = "NFC_SCAN"]
    NFCScan,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(belongs_to(WaterCloset))]
#[diesel(table_name = crate::schema::histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct History {
    pub id: i32,
    pub water_closet_id: i32,
    pub datetime: NaiveDateTime,
    pub action: Actions,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(WaterCloset))]
#[diesel(table_name = crate::schema::histories)]
pub struct NewHistory {
    pub water_closet_id: i32,
    pub datetime: NaiveDateTime,
    pub action: Actions,
}
