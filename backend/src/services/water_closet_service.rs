use crate::models::database::DatabaseConnection;
use crate::models::water_closet::{NewWaterCloset, WaterCloset};
use crate::schema::water_closets::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_water_closet(
    conn: &mut DatabaseConnection,
    new_water_closet: &NewWaterCloset,
) -> Result<WaterCloset, Error> {

    let new_water_closet = NewWaterCloset {
        group_id: new_water_closet.group_id,
        is_disabled: new_water_closet.is_disabled,
        is_available: new_water_closet.is_available,
        is_door_opened: new_water_closet.is_door_opened,
        is_door_locked: new_water_closet.is_door_locked,
        clean_state: new_water_closet.clean_state.clone(),
    };

    diesel::insert_into(water_closets)
        .values(&new_water_closet)
        .get_result::<WaterCloset>(conn)
}

pub async fn get_water_closets(conn: &mut DatabaseConnection) -> Result<Vec<WaterCloset>, Error> {
    water_closets.load::<WaterCloset>(conn)
}

pub async fn get_water_closet(
    conn: &mut DatabaseConnection,
    water_closet_id: i32,
) -> Result<WaterCloset, Error> {
    water_closets
        .filter(id.eq(water_closet_id))
        .first::<WaterCloset>(conn)
}

pub async fn update_water_closet(
    conn: &mut DatabaseConnection,
    water_closet_id: i32,
    updated_water_closet: &NewWaterCloset,
) -> Result<WaterCloset, Error> {
    diesel::update(water_closets.find(water_closet_id))
        .set((
            group_id.eq(&updated_water_closet.group_id),
            is_disabled.eq(&updated_water_closet.is_disabled),
            is_available.eq(&updated_water_closet.is_available),
            is_door_opened.eq(&updated_water_closet.is_door_opened),
            is_door_locked.eq(&updated_water_closet.is_door_locked),
            clean_state.eq(&updated_water_closet.clean_state),
        ))
        .get_result(conn)
}

pub async fn delete_water_closet(
    conn: &mut DatabaseConnection,
    water_closet_id: i32,
) -> Result<WaterCloset, Error> {
    diesel::delete(water_closets.find(water_closet_id))
        .get_result(conn)
}

pub async fn get_water_closets_by_group_id(
    conn: &mut DatabaseConnection,
    id_group: i32,
) -> Result<Vec<WaterCloset>, Error> {
    water_closets
        .filter(crate::schema::water_closets::group_id.eq(id_group))
        .load::<WaterCloset>(conn)
}
