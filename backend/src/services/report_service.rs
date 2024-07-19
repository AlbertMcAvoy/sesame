use crate::models::report::{NewReport, Report, ReportDTO};
use crate::models::user::User;
use crate::schema::reports::dsl::*;
use crate::schema::users::dsl::{mail, users};
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn create_report(
    app_state: &web::Data<AppState>,
    new_report: &ReportDTO,
    user_mail: String,
) -> Result<Report, String> {
    let mut conn = app_state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    let user = users
        .filter(mail.eq(user_mail))
        .first::<User>(&mut conn)
        .unwrap();

    let new_report = NewReport {
        user_id: user.id,
        water_closet_id: new_report.water_closet_id,
        datetime: new_report.datetime,
        state: new_report.state.clone(),
        topic: new_report.topic.clone(),
        comment: new_report.comment.clone(),
    };

    diesel::insert_into(reports)
        .values(&new_report)
        .get_result::<Report>(&mut conn)
        .map_err(|err| format!("Failed to insert report: {}", err))
}

pub async fn get_reports(app_state: &web::Data<AppState>) -> Result<Vec<Report>, String> {
    let mut conn = app_state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    reports
        .load::<Report>(&mut conn)
        .map_err(|err| format!("Failed to load reports: {}", err))
}

pub async fn get_report(app_state: &web::Data<AppState>, report_id: i32) -> Result<Report, String> {
    let mut conn = app_state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    reports
        .filter(id.eq(report_id))
        .first::<Report>(&mut conn)
        .map_err(|err| format!("Report not found: {}", err))
}

pub async fn update_report(
    app_state: &web::Data<AppState>,
    report_id: i32,
    updated_report: &NewReport,
) -> Result<(), String> {
    let mut conn = app_state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::update(reports.find(report_id))
        .set((
            user_id.eq(&updated_report.user_id),
            water_closet_id.eq(&updated_report.water_closet_id),
            datetime.eq(&updated_report.datetime),
            state.eq(&updated_report.state),
            topic.eq(&updated_report.topic),
            comment.eq(&updated_report.comment),
        ))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to update report: {}", err))
}

pub async fn delete_report(app_state: &web::Data<AppState>, report_id: i32) -> Result<(), String> {
    let mut conn = app_state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::delete(reports.find(report_id))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to delete report: {}", err))
}
