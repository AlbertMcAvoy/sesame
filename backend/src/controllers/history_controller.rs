use crate::{helpers::errors::web_errors, models::history::NewHistory};
use crate::services::history_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_history(
    state: web::Data<AppState>,
    new_history: web::Json<NewHistory>,
) -> impl Responder {
    match history_service::create_history(&mut state.get_ref().get_conn(), &new_history).await {
        Ok(history) => HttpResponse::Created().json(history),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_histories(state: web::Data<AppState>) -> impl Responder {
    match history_service::get_histories(&mut state.get_ref().get_conn()).await {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError()    
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_history(state: web::Data<AppState>, history_id: web::Path<i32>) -> impl Responder {
    match history_service::get_history(&mut state.get_ref().get_conn(), *history_id).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::NotFound()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 404, msg: err.to_string() })
    }
}

pub async fn update_history(
    state: web::Data<AppState>,
    history_id: web::Path<i32>,
    updated_history: web::Json<NewHistory>,
) -> impl Responder {
    match history_service::update_history(&mut state.get_ref().get_conn(), *history_id, &updated_history).await {
        Ok(_) => HttpResponse::Ok().json(updated_history.into_inner()),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn delete_history(
    state: web::Data<AppState>,
    history_id: web::Path<i32>,
) -> impl Responder {
    match history_service::delete_history(&mut state.get_ref().get_conn(), *history_id).await {
        Ok(_) => HttpResponse::Ok().body("History deleted"),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}
