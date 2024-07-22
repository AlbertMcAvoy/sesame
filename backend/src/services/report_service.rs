use crate::models::database::DatabaseConnection;
use crate::models::report::{NewReport, Report, ReportDTO};
use crate::models::user::User;
use crate::schema::reports::dsl::*;
use crate::schema::users::dsl::{mail, users};
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_report(
    conn: &mut DatabaseConnection,
    new_report: &ReportDTO,
    user_mail: String,
) -> Result<Report, Error> {
    let user = users
        .filter(mail.eq(user_mail))
        .first::<User>(conn)
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
        .get_result::<Report>(conn)
}

pub async fn get_reports(conn: &mut DatabaseConnection) -> Result<Vec<Report>, Error> {
    reports
        .load::<Report>(conn)
}

pub async fn get_report(conn: &mut DatabaseConnection, report_id: i32) -> Result<Report, Error> {
    reports
        .filter(id.eq(report_id))
        .first::<Report>(conn)
}

pub async fn update_report(
    conn: &mut DatabaseConnection,
    report_id: i32,
    updated_report: &NewReport,
) -> Result<Report, Error> {
    diesel::update(reports.find(report_id))
        .set((
            user_id.eq(&updated_report.user_id),
            water_closet_id.eq(&updated_report.water_closet_id),
            datetime.eq(&updated_report.datetime),
            state.eq(&updated_report.state),
            topic.eq(&updated_report.topic),
            comment.eq(&updated_report.comment),
        ))
        .get_result(conn)
}

pub async fn delete_report(conn: &mut DatabaseConnection, report_id: i32) -> Result<Report, Error> {
    diesel::delete(reports.find(report_id))
        .get_result(conn)
}
