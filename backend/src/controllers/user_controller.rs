use crate::{helpers::errors::web_errors, models::user::NewUser};
use crate::services::user_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_user(
    state: web::Data<AppState>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    match user_service::create_user(&mut state.get_ref().get_conn(), &new_user).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_users(state: web::Data<AppState>) -> impl Responder {
    match user_service::get_users(&mut state.get_ref().get_conn()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_user(state: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    match user_service::get_user(&mut state.get_ref().get_conn(), *user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::NotFound()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 404, msg: err.to_string() })
    }
}

pub async fn update_user(
    state: web::Data<AppState>,
    user_id: web::Path<i32>,
    updated_user: web::Json<NewUser>,
) -> impl Responder {
    match user_service::update_user(&mut state.get_ref().get_conn(), *user_id, &updated_user).await {
        Ok(_) => HttpResponse::Ok().json(updated_user.into_inner()),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn delete_user(state: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    match user_service::delete_user(&mut state.get_ref().get_conn(), *user_id).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}
