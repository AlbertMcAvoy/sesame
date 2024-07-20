use crate::{helpers::errors::web_errors, services::auth_service};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MailInput {
    mail: String,
}

pub async fn authentification(
    state: web::Data<AppState>,
    new_mail: web::Json<MailInput>,
) -> impl Responder {
    match auth_service::authentificate(&mut state.get_ref().get_conn(), &new_mail.mail).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::InternalServerError()
            .content_type("application/json;charset=utf-8")
            .json(web_errors::ErrorResult { status_code: 500, msg: err.to_string() }),
    }
}
