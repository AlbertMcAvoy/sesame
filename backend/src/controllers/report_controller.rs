use crate::models::report::NewReport;
use crate::services::report_service;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_report(
    app_state: web::Data<AppState>,
    new_report: web::Json<NewReport>,
) -> impl Responder {
    match report_service::create_report(&app_state, &new_report).await {
        Ok(report) => HttpResponse::Created().json(report),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert report: {}", err))
        }
    }
}

pub async fn get_reports(app_state: web::Data<AppState>) -> impl Responder {
    match report_service::get_reports(&app_state).await {
        Ok(reports) => HttpResponse::Ok().json(reports),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load reports: {}", err))
        }
    }
}

pub async fn get_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
) -> impl Responder {
    match report_service::get_report(&app_state, *report_id).await {
        Ok(report) => HttpResponse::Ok().json(report),
        Err(err) => HttpResponse::NotFound().body(format!("Report not found: {}", err)),
    }
}

pub async fn update_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
    updated_report: web::Json<NewReport>,
) -> impl Responder {
    match report_service::update_report(&app_state, *report_id, &updated_report).await {
        Ok(_) => HttpResponse::Ok().json(updated_report.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update report: {}", err))
        }
    }
}

pub async fn delete_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
) -> impl Responder {
    match report_service::delete_report(&app_state, *report_id).await {
        Ok(_) => HttpResponse::Ok().body("Report deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete report: {}", err))
        }
    }
}
