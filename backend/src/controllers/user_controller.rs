use crate::models::user::NewUser;
use crate::services::user_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_user(
    state: web::Data<AppState>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    match user_service::create_user(&state, &new_user).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert user: {}", err))
        }
    }
}

pub async fn get_users(state: web::Data<AppState>) -> impl Responder {
    match user_service::get_users(&state).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load users: {}", err))
        }
    }
}

pub async fn get_user(state: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    match user_service::get_user(&state, *user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::NotFound().body(format!("User not found: {}", err)),
    }
}

pub async fn update_user(
    state: web::Data<AppState>,
    user_id: web::Path<i32>,
    updated_user: web::Json<NewUser>,
) -> impl Responder {
    match user_service::update_user(&state, *user_id, &updated_user).await {
        Ok(_) => HttpResponse::Ok().json(updated_user.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update user: {}", err))
        }
    }
}

pub async fn delete_user(state: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    match user_service::delete_user(&state, *user_id).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete user: {}", err))
        }
    }
}
