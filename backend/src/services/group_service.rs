// Au minimum, on a besoin des services suivants :
// TODO: Get_groups

use crate::models::group::{Group, NewGroup};
use crate::schema::groups::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn create_group(
    state: &web::Data<AppState>,
    new_group: &NewGroup,
) -> Result<Group, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    let new_group = NewGroup {
        user_id: new_group.user_id,
        name: new_group.name.clone(),
    };

    diesel::insert_into(groups)
        .values(&new_group)
        .get_result::<Group>(&mut conn)
        .map_err(|err| format!("Failed to insert group: {}", err))
}

pub async fn get_groups(state: &web::Data<AppState>) -> Result<Vec<Group>, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    groups
        .load::<Group>(&mut conn)
        .map_err(|err| format!("Failed to load groups: {}", err))
}

pub async fn get_group(state: &web::Data<AppState>, group_id: i32) -> Result<Group, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    groups
        .filter(id.eq(group_id))
        .first::<Group>(&mut conn)
        .map_err(|err| format!("Group not found: {}", err))
}

pub async fn update_group(
    state: &web::Data<AppState>,
    group_id: i32,
    updated_group: &NewGroup,
) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::update(groups.find(group_id))
        .set((
            user_id.eq(&updated_group.user_id),
            name.eq(&updated_group.name),
        ))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to update group: {}", err))
}

pub async fn delete_group(state: &web::Data<AppState>, group_id: i32) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::delete(groups.find(group_id))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to delete group: {}", err))
}
