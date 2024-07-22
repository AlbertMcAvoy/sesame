use crate::{helpers::errors::web_errors, models::water_closet::NewWaterCloset};
use crate::services::water_closet_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_water_closet(
    state: web::Data<AppState>,
    new_water_closet: web::Json<NewWaterCloset>,
) -> impl Responder {
    match water_closet_service::create_water_closet(&mut state.get_ref().get_conn(), &new_water_closet).await {
        Ok(water_closet) => HttpResponse::Created().json(water_closet),
        Err(err) => HttpResponse::InternalServerError()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_water_closets(state: web::Data<AppState>) -> impl Responder {
    match water_closet_service::get_water_closets(&mut state.get_ref().get_conn()).await {
        Ok(water_closets) => HttpResponse::Ok().json(water_closets),
        Err(err) => HttpResponse::InternalServerError()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
) -> impl Responder {
    match water_closet_service::get_water_closet(&mut state.get_ref().get_conn(), *water_closet_id).await {
        Ok(water_closet) => HttpResponse::Ok().json(water_closet),
        Err(err) => HttpResponse::NotFound()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 404, msg: err.to_string() })
    }
}

pub async fn update_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
    updated_water_closet: web::Json<NewWaterCloset>,
) -> impl Responder {
    match water_closet_service::update_water_closet(&mut state.get_ref().get_conn(), *water_closet_id, &updated_water_closet)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(updated_water_closet.into_inner()),
        Err(err) => HttpResponse::InternalServerError()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn delete_water_closet(
    state: web::Data<AppState>,
    water_closet_id: web::Path<i32>,
) -> impl Responder {
    match water_closet_service::delete_water_closet(&mut state.get_ref().get_conn(), *water_closet_id).await {
        Ok(_) => HttpResponse::Ok().body("Water closet deleted"),
        Err(err) => HttpResponse::InternalServerError()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_water_closets_by_group(
    state: web::Data<AppState>,
    group_id: web::Path<i32>,
) -> impl Responder {
    match water_closet_service::get_water_closets_by_group_id(&mut state.get_ref().get_conn(), *group_id).await {
        Ok(water_closets) => HttpResponse::Ok().json(water_closets),
        Err(err) => HttpResponse::InternalServerError()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}
