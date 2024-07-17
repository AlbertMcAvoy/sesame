use crate::models::group::Group;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(belongs_to(Group))]
#[diesel(table_name = crate::schema::places)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Place {
    pub id: i32,
    pub group_id: i32,
    pub coordonates: String,
}
