// Au minimum, on a besoin des services suivants :
// TODO: Get_groups

use crate::models::database::DatabaseConnection;
use crate::models::group::{Group, GroupDTO, NewGroup};
use crate::models::place::Place;
use crate::schema::groups::dsl::*;
use crate::schema::places::dsl::{group_id, places};
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_group(
    conn: &mut DatabaseConnection,
    new_group: &NewGroup,
) -> Result<Group, Error> {
    
    let new_group = NewGroup {
        user_id: new_group.user_id,
        name: new_group.name.clone(),
    };

    diesel::insert_into(groups)
    .values(&new_group)
    .get_result::<Group>(conn)
}

pub async fn get_groups(conn: &mut DatabaseConnection) -> Result<Vec<GroupDTO>, Error> {

    match groups
        .load::<Group>(conn)
    {
        Ok(groups_res) => {
            let mut res_groups: Vec<GroupDTO> = Vec::new();
            for group in groups_res.iter() {
                match places
                    .filter(group_id.eq(group.id))
                    .first::<Place>(conn)
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

pub async fn get_group(conn: &mut DatabaseConnection, id_group: i32) -> Result<Group, Error> {

    groups
        .filter(id.eq(id_group))
        .first::<Group>(conn)
}

pub async fn update_group(
    conn: &mut DatabaseConnection,
    id_group: i32,
    updated_group: &NewGroup,
) -> Result<Group, Error> {

    diesel::update(groups.find(id_group))
        .set((
            user_id.eq(&updated_group.user_id),
            name.eq(&updated_group.name),
        ))
        .get_result(conn)
}

pub async fn delete_group(conn: &mut DatabaseConnection, id_group: i32) -> Result<Group, Error> {

    diesel::delete(groups.find(id_group))
        .get_result(conn)
}
