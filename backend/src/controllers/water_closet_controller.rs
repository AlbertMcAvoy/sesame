use crate::models::water_closet::NewWaterCloset;
use crate::services::water_closet_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_water_closet(
    state: web::Data<AppState>,
    new_water_closet: web::Json<NewWaterCloset>,
) -> impl Responder {
    match water_closet_service::create_water_closet(&state, &new_water_closet).await {
        Ok(water_closet) => HttpResponse::Created().json(water_closet),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to insert water closet: {}", err)),
    }
}

pub async fn get_water_closets(state: web::Data<AppState>) -> impl Responder {
    match water_closet_service::get_water_closets(&state).await {
        Ok(water_closets) => HttpResponse::Ok().json(water_closets),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to load water closets: {}", err)),
    }
}

pub async fn get_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
) -> impl Responder {
    match water_closet_service::get_water_closet(&state, *water_closet_id).await {
        Ok(water_closet) => HttpResponse::Ok().json(water_closet),
        Err(err) => HttpResponse::NotFound().body(format!("Water closet not found: {}", err)),
    }
}

pub async fn update_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
    updated_water_closet: web::Json<NewWaterCloset>,
) -> impl Responder {
    match water_closet_service::update_water_closet(&state, *water_closet_id, &updated_water_closet)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(updated_water_closet.into_inner()),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to update water closet: {}", err)),
    }
}

pub async fn delete_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
) -> impl Responder {
    match water_closet_service::delete_water_closet(&state, *water_closet_id).await {
        Ok(_) => HttpResponse::Ok().body("Water closet deleted"),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to delete water closet: {}", err)),
    }
}
