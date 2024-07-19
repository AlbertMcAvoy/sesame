// Au minimum, on a besoin des services suivants :
// TODO: Get_groups

use crate::models::group::{Group, GroupDTO, NewGroup};
use crate::models::place::Place;
use crate::schema::groups::dsl::*;
use crate::schema::places::dsl::{group_id, places};
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

pub async fn get_groups(state: &web::Data<AppState>) -> Result<Vec<GroupDTO>, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    match groups
        .load::<Group>(&mut conn)
        .map_err(|err| format!("Failed to load groups: {}", err))
    {
        Ok(groups_res) => {
            let mut res_groups: Vec<GroupDTO> = Vec::new();
            for group in groups_res.iter() {
                match places
                    .filter(group_id.eq(group.id))
                    .first::<Place>(&mut conn)
                {
                    Ok(place) => res_groups.push(GroupDTO {
                        id: group.id.clone(),
                        user_id: group.user_id.clone(),
                        name: group.name.clone(),
                        place: Some(place),
                    }),
                    Err(_) => res_groups.push(GroupDTO {
                        id: group.id.clone(),
                        user_id: group.user_id.clone(),
                        name: group.name.clone(),
                        place: None,
                    }),
                };
            }
            Ok(res_groups)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_group(state: &web::Data<AppState>, id_group: i32) -> Result<Group, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    groups
        .filter(id.eq(id_group))
        .first::<Group>(&mut conn)
        .map_err(|err| format!("Group not found: {}", err))
}

pub async fn update_group(
    state: &web::Data<AppState>,
    id_group: i32,
    updated_group: &NewGroup,
) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::update(groups.find(id_group))
        .set((
            user_id.eq(&updated_group.user_id),
            name.eq(&updated_group.name),
        ))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to update group: {}", err))
}

pub async fn delete_group(state: &web::Data<AppState>, id_group: i32) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::delete(groups.find(id_group))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to delete group: {}", err))
}
