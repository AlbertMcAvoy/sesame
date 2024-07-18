// TODO: Get_group_waterClosets(group_id)

use crate::models::group::NewGroup;
use crate::services::group_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_group(
    state: web::Data<AppState>,
    new_group: web::Json<NewGroup>,
) -> impl Responder {
    match group_service::create_group(&state, &new_group).await {
        Ok(group) => HttpResponse::Created().json(group),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert group: {}", err))
        }
    }
}

pub async fn get_groups(state: web::Data<AppState>) -> impl Responder {
    match group_service::get_groups(&state).await {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load groups: {}", err))
        }
    }
}

pub async fn get_group(state: web::Data<AppState>, group_id: web::Path<i32>) -> impl Responder {
    match group_service::get_group(&state, *group_id).await {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(err) => HttpResponse::NotFound().body(format!("Group not found: {}", err)),
    }
}

pub async fn update_group(
    state: web::Data<AppState>,
    group_id: web::Path<i32>,
    updated_group: web::Json<NewGroup>,
) -> impl Responder {
    match group_service::update_group(&state, *group_id, &updated_group).await {
        Ok(_) => HttpResponse::Ok().json(updated_group.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update group: {}", err))
        }
    }
}

pub async fn delete_group(state: web::Data<AppState>, group_id: web::Path<i32>) -> impl Responder {
    match group_service::delete_group(&state, *group_id).await {
        Ok(_) => HttpResponse::Ok().body("Group deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete group: {}", err))
        }
    }
}
