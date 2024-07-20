use crate::helpers::errors::web_errors;
use crate::models::report::{NewReport, ReportDTO};
use crate::services::{auth_service::get_mail_from_token, report_service};
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorization")?.to_str().ok()
}

pub async fn create_report(
    app_state: web::Data<AppState>,
    new_report: web::Json<ReportDTO>,
    req: HttpRequest,
) -> impl Responder {
    match get_content_type(&req) {
        Some(auth_token) => match get_mail_from_token(auth_token) {
            Ok(mail) => match report_service::create_report(&mut app_state.get_ref().get_conn(), &new_report, mail).await {
                Ok(report) => HttpResponse::Created().json(report),
                Err(err) => HttpResponse::InternalServerError()
                        .content_type("application/json;charset=utf-8")
                        .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
            },
            Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
        },
        None => HttpResponse::BadRequest().finish(),
    }
}

pub async fn get_reports(app_state: web::Data<AppState>) -> impl Responder {
    match report_service::get_reports(&mut app_state.get_ref().get_conn()).await {
        Ok(reports) => HttpResponse::Ok().json(reports),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn get_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
) -> impl Responder {
    match report_service::get_report(&mut app_state.get_ref().get_conn(), *report_id).await {
        Ok(report) => HttpResponse::Ok().json(report),
        Err(err) => HttpResponse::NotFound()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 404, msg: err.to_string() })
    }
}

pub async fn update_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
    updated_report: web::Json<NewReport>,
) -> impl Responder {
    match report_service::update_report(&mut app_state.get_ref().get_conn(), *report_id, &updated_report).await {
        Ok(_) => HttpResponse::Ok().json(updated_report.into_inner()),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}

pub async fn delete_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
) -> impl Responder {
    match report_service::delete_report(&mut app_state.get_ref().get_conn(), *report_id).await {
        Ok(_) => HttpResponse::Ok().body("Report deleted"),
        Err(err) => HttpResponse::InternalServerError()
                .content_type("application/json;charset=utf-8")
                .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() })
    }
}
