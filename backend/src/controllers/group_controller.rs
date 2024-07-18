// TODO: Get_group_waterClosets(group_id)

use crate::models::group::{Group, NewGroup};
use crate::schema::groups::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn create_group(
    state: web::Data<AppState>,
    new_group: web::Json<NewGroup>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_group = NewGroup {
        user_id: new_group.user_id,
        name: new_group.name.clone(),
    };

    match diesel::insert_into(groups)
        .values(&new_group)
        .get_result::<Group>(&mut conn)
    {
        Ok(inserted_group) => HttpResponse::Created().json(inserted_group),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert group: {}", err))
        }
    }
}

pub async fn get_groups(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match groups.load::<Group>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load groups: {}", err))
        }
    }
}

pub async fn get_group(state: web::Data<AppState>, group_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match groups.filter(id.eq(*group_id)).first::<Group>(&mut conn) {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(err) => HttpResponse::NotFound().body(format!("Group not found: {}", err)),
    }
}

pub async fn update_group(
    state: web::Data<AppState>,
    group_id: web::Path<i32>,
    updated_group: web::Json<NewGroup>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(groups.find(*group_id))
        .set((
            user_id.eq(&updated_group.user_id),
            name.eq(&updated_group.name),
        ))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_group.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update group: {}", err))
        }
    }
}

pub async fn delete_group(state: web::Data<AppState>, group_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(groups.find(*group_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().body("Group deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete group: {}", err))
        }
    }
}
