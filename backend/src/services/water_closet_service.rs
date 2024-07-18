use crate::models::water_closet::{NewWaterCloset, WaterCloset};
use crate::schema::water_closets::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn create_water_closet(
    state: &web::Data<AppState>,
    new_water_closet: &NewWaterCloset,
) -> Result<WaterCloset, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

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
        .get_result::<WaterCloset>(&mut conn)
        .map_err(|err| format!("Failed to insert water closet: {}", err))
}

pub async fn get_water_closets(state: &web::Data<AppState>) -> Result<Vec<WaterCloset>, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    water_closets
        .load::<WaterCloset>(&mut conn)
        .map_err(|err| format!("Failed to load water closets: {}", err))
}

pub async fn get_water_closet(
    state: &web::Data<AppState>,
    water_closet_id: i32,
) -> Result<WaterCloset, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    water_closets
        .filter(id.eq(water_closet_id))
        .first::<WaterCloset>(&mut conn)
        .map_err(|err| format!("Water closet not found: {}", err))
}

pub async fn update_water_closet(
    state: &web::Data<AppState>,
    water_closet_id: i32,
    updated_water_closet: &NewWaterCloset,
) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::update(water_closets.find(water_closet_id))
        .set((
            group_id.eq(&updated_water_closet.group_id),
            is_disabled.eq(&updated_water_closet.is_disabled),
            is_available.eq(&updated_water_closet.is_available),
            is_door_opened.eq(&updated_water_closet.is_door_opened),
            is_door_locked.eq(&updated_water_closet.is_door_locked),
            clean_state.eq(&updated_water_closet.clean_state),
        ))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to update water closet: {}", err))
}

pub async fn delete_water_closet(
    state: &web::Data<AppState>,
    water_closet_id: i32,
) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::delete(water_closets.find(water_closet_id))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to delete water closet: {}", err))
}
