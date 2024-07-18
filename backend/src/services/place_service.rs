use crate::models::place::{NewPlace, Place};
use crate::schema::places::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn create_place(
    state: &web::Data<AppState>,
    new_place: &NewPlace,
) -> Result<Place, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    let new_place = NewPlace {
        group_id: new_place.group_id,
        coordonates: new_place.coordonates.clone(),
    };

    diesel::insert_into(places)
        .values(&new_place)
        .get_result::<Place>(&mut conn)
        .map_err(|err| format!("Failed to insert place: {}", err))
}

pub async fn get_places(state: &web::Data<AppState>) -> Result<Vec<Place>, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    places
        .load::<Place>(&mut conn)
        .map_err(|err| format!("Failed to load places: {}", err))
}

pub async fn get_place(state: &web::Data<AppState>, place_id: i32) -> Result<Place, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    places
        .filter(id.eq(place_id))
        .first::<Place>(&mut conn)
        .map_err(|err| format!("Place not found: {}", err))
}

pub async fn update_place(
    state: &web::Data<AppState>,
    place_id: i32,
    updated_place: &NewPlace,
) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::update(places.find(place_id))
        .set((
            group_id.eq(&updated_place.group_id),
            coordonates.eq(&updated_place.coordonates),
        ))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to update place: {}", err))
}

pub async fn delete_place(state: &web::Data<AppState>, place_id: i32) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::delete(places.find(place_id))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to delete place: {}", err))
}
