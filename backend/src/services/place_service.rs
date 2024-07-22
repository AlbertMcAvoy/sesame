use crate::models::database::DatabaseConnection;
use crate::models::place::{NewPlace, Place};
use crate::schema::places::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_place(
    conn: &mut DatabaseConnection,
    new_place: &NewPlace,
) -> Result<Place, Error> {

    let new_place = NewPlace {
        group_id: new_place.group_id,
        coordonates: new_place.coordonates.clone(),
    };

    diesel::insert_into(places)
        .values(&new_place)
        .get_result::<Place>(conn)
}

pub async fn get_places(conn: &mut DatabaseConnection) -> Result<Vec<Place>, Error> {
    places.load::<Place>(conn)
}

pub async fn get_place(conn: &mut DatabaseConnection, place_id: i32) -> Result<Place, Error> {
    places
        .filter(id.eq(place_id))
        .first::<Place>(conn)
}

pub async fn update_place(
    conn: &mut DatabaseConnection,
    place_id: i32,
    updated_place: &NewPlace,
) -> Result<Place, Error> {
    diesel::update(places.find(place_id))
        .set((
            group_id.eq(&updated_place.group_id),
            coordonates.eq(&updated_place.coordonates),
        ))
        .get_result(conn)
}

pub async fn delete_place(conn: &mut DatabaseConnection, place_id: i32) -> Result<Place, Error> {
    diesel::delete(places.find(place_id))
        .get_result(conn)
}
