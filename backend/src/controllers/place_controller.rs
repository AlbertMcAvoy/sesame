use crate::models::place::NewPlace;
use crate::services::place_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_place(
    state: web::Data<AppState>,
    new_place: web::Json<NewPlace>,
) -> impl Responder {
    match place_service::create_place(&state, &new_place).await {
        Ok(place) => HttpResponse::Created().json(place),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert place: {}", err))
        }
    }
}

pub async fn get_places(state: web::Data<AppState>) -> impl Responder {
    match place_service::get_places(&state).await {
        Ok(places) => HttpResponse::Ok().json(places),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load places: {}", err))
        }
    }
}

pub async fn get_place(state: web::Data<AppState>, place_id: web::Path<i32>) -> impl Responder {
    match place_service::get_place(&state, *place_id).await {
        Ok(place) => HttpResponse::Ok().json(place),
        Err(err) => HttpResponse::NotFound().body(format!("Place not found: {}", err)),
    }
}

pub async fn update_place(
    state: web::Data<AppState>,
    place_id: web::Path<i32>,
    updated_place: web::Json<NewPlace>,
) -> impl Responder {
    match place_service::update_place(&state, *place_id, &updated_place).await {
        Ok(_) => HttpResponse::Ok().json(updated_place.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update place: {}", err))
        }
    }
}

pub async fn delete_place(state: web::Data<AppState>, place_id: web::Path<i32>) -> impl Responder {
    match place_service::delete_place(&state, *place_id).await {
        Ok(_) => HttpResponse::Ok().body("Place deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete place: {}", err))
        }
    }
}
