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
