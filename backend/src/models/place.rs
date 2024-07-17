use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::places)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Place {
    pub id: i32,
    pub group_id: i32,
    pub coordonates: String,
}
