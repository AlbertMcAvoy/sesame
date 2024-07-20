use crate::{helpers::errors::web_errors, models::group::NewGroup};
use crate::services::group_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_group(
    state: web::Data<AppState>,
    new_group: web::Json<NewGroup>,
) -> impl Responder {
    match group_service::create_group(&mut state.get_ref().get_conn(), &new_group).await {
        Ok(group) => HttpResponse::Created().json(group),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_groups(state: web::Data<AppState>) -> impl Responder {
    match group_service::get_groups(&mut state.get_ref().get_conn()).await {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_group(state: web::Data<AppState>, group_id: web::Path<i32>) -> impl Responder {
    match group_service::get_group(&mut state.get_ref().get_conn(), *group_id).await {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(err) => HttpResponse::NotFound()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 404, msg: err.to_string() })
    }
}

pub async fn update_group(
    state: web::Data<AppState>,
    group_id: web::Path<i32>,
    updated_group: web::Json<NewGroup>,
) -> impl Responder {
    match group_service::update_group(&mut state.get_ref().get_conn(), *group_id, &updated_group).await {
        Ok(_) => HttpResponse::Ok().json(updated_group.into_inner()),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn delete_group(state: web::Data<AppState>, group_id: web::Path<i32>) -> impl Responder {
    match group_service::delete_group(&mut state.get_ref().get_conn(), *group_id).await {
        Ok(_) => HttpResponse::Ok().body("Group deleted"),
        Err(err) => HttpResponse::InternalServerError()    
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}
