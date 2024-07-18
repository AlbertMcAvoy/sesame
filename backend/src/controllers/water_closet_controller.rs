use crate::models::water_closet::{NewWaterCloset, WaterCloset};
use crate::schema::water_closets::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

// Créer une entrée de WaterCloset
pub async fn create_water_closet(
    state: web::Data<AppState>,
    new_water_closet: web::Json<NewWaterCloset>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_water_closet = NewWaterCloset {
        group_id: new_water_closet.group_id,
        is_disabled: new_water_closet.is_disabled,
        is_door_opened: new_water_closet.is_door_opened,
        is_door_locked: new_water_closet.is_door_locked,
        clean_state: new_water_closet.clean_state.clone(),
    };

    match diesel::insert_into(water_closets)
        .values(&new_water_closet)
        .get_result::<WaterCloset>(&mut conn)
    {
        Ok(inserted_water_closet) => HttpResponse::Created().json(inserted_water_closet),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to insert water closet: {}", err)),
    }
}

// Lire toutes les entrées de WaterCloset
pub async fn get_water_closets(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match water_closets.load::<WaterCloset>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to load water closets: {}", err)),
    }
}

// Lire une entrée spécifique de WaterCloset
pub async fn get_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match water_closets
        .filter(id.eq(*water_closet_id))
        .first::<WaterCloset>(&mut conn)
    {
        Ok(water_closet) => HttpResponse::Ok().json(water_closet),
        Err(err) => HttpResponse::NotFound().body(format!("Water closet not found: {}", err)),
    }
}

// Mettre à jour une entrée de WaterCloset
pub async fn update_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
    updated_water_closet: web::Json<NewWaterCloset>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(water_closets.find(*water_closet_id))
        .set((
            group_id.eq(&updated_water_closet.group_id),
            is_disabled.eq(&updated_water_closet.is_disabled),
            is_door_opened.eq(&updated_water_closet.is_door_opened),
            is_door_locked.eq(&updated_water_closet.is_door_locked),
            clean_state.eq(&updated_water_closet.clean_state),
        ))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_water_closet.into_inner()),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to update water closet: {}", err)),
    }
}

// Supprimer une entrée de WaterCloset
pub async fn delete_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(water_closets.find(*water_closet_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().body("Water closet deleted"),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to delete water closet: {}", err)),
    }
}
