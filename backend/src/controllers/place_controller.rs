use crate::models::place::{NewPlace, Place};
use crate::schema::places::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

// Créer une entrée de place
pub async fn create_place(
    state: web::Data<AppState>,
    new_place: web::Json<NewPlace>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_place = NewPlace {
        group_id: new_place.group_id,
        coordonates: new_place.coordonates.clone(),
    };

    match diesel::insert_into(places)
        .values(&new_place)
        .get_result::<Place>(&mut conn)
    {
        Ok(inserted_place) => HttpResponse::Created().json(inserted_place),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert place: {}", err))
        }
    }
}

// Lire toutes les entrées de place
pub async fn get_places(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match places.load::<Place>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load places: {}", err))
        }
    }
}

// Lire une entrée spécifique de place
pub async fn get_place(state: web::Data<AppState>, place_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match places.filter(id.eq(*place_id)).first::<Place>(&mut conn) {
        Ok(place) => HttpResponse::Ok().json(place),
        Err(err) => HttpResponse::NotFound().body(format!("Place not found: {}", err)),
    }
}

// Mettre à jour une entrée de place
pub async fn update_place(
    state: web::Data<AppState>,
    place_id: web::Path<i32>,
    updated_place: web::Json<NewPlace>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(places.find(*place_id))
        .set((
            group_id.eq(&updated_place.group_id),
            coordonates.eq(&updated_place.coordonates),
        ))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_place.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update place: {}", err))
        }
    }
}

// Supprimer une entrée de place
pub async fn delete_place(state: web::Data<AppState>, place_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(places.find(*place_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().body("Place deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete place: {}", err))
        }
    }
}
