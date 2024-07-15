use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{models::categories::NewCategory, services::categories_service, AppState};

pub async fn create_category(
    state: web::Data<AppState>,
    new_category: web::Json<NewCategory>,
) -> impl Responder {
    match categories_service::create_category(&mut state.get_conn(), new_category.into_inner()).await
    {
        Ok(inserted_category) => HttpResponse::Created().json(inserted_category),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert category: {}", err))
        }
    }
}

pub async fn update_category(
    state: web::Data<AppState>,
    new_category: web::Json<NewCategory>,
    category_id: web::Path<Uuid>
) -> impl Responder {
    match categories_service::update_category(&mut state.get_conn(), new_category.into_inner(), category_id.into_inner()).await
    {
        Ok(inserted_category) => HttpResponse::Ok().json(inserted_category),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert category: {}", err))
        }
    }
}

pub async fn delete_category(state: web::Data<AppState>, category_id: web::Path<Uuid>) -> impl Responder {
    match categories_service::delete_category(&mut state.get_conn(), category_id.into_inner()).await {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(err) => HttpResponse::InternalServerError().body(format!("An error occured: {}", err)),
    }
}

pub async fn get_categories(state: web::Data<AppState>) -> impl Responder {
    match categories_service::get_categories(&mut state.get_conn()).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load categories: {}", err))
        }
    }
}