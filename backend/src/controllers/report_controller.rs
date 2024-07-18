use crate::models::report::{NewReport, Report};
use crate::schema::reports::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

// Créer une entrée de report
pub async fn create_report(
    app_state: web::Data<AppState>,
    new_report: web::Json<NewReport>,
) -> impl Responder {
    let mut conn = app_state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_report = NewReport {
        user_id: new_report.user_id,
        water_closet_id: new_report.water_closet_id,
        datetime: new_report.datetime,
        state: new_report.state.clone(),
        topic: new_report.topic.clone(),
        comment: new_report.comment.clone(),
    };

    match diesel::insert_into(reports)
        .values(&new_report)
        .get_result::<Report>(&mut conn)
    {
        Ok(inserted_report) => HttpResponse::Created().json(inserted_report),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert report: {}", err))
        }
    }
}

// Lire toutes les entrées de report
pub async fn get_reports(app_state: web::Data<AppState>) -> impl Responder {
    let mut conn = app_state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match reports.load::<Report>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load reports: {}", err))
        }
    }
}

// Lire une entrée spécifique de report
pub async fn get_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = app_state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match reports.filter(id.eq(*report_id)).first::<Report>(&mut conn) {
        Ok(report) => HttpResponse::Ok().json(report),
        Err(err) => HttpResponse::NotFound().body(format!("Report not found: {}", err)),
    }
}

// Mettre à jour une entrée de report
pub async fn update_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
    updated_report: web::Json<NewReport>,
) -> impl Responder {
    let mut conn = app_state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(reports.find(*report_id))
        .set((
            user_id.eq(&updated_report.user_id),
            water_closet_id.eq(&updated_report.water_closet_id),
            datetime.eq(&updated_report.datetime),
            state.eq(&updated_report.state),
            topic.eq(&updated_report.topic),
            comment.eq(&updated_report.comment),
        ))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_report.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update report: {}", err))
        }
    }
}

// Supprimer une entrée de report
pub async fn delete_report(
    app_state: web::Data<AppState>,
    report_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = app_state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(reports.find(*report_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().body("Report deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete report: {}", err))
        }
    }
}
