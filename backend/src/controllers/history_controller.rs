use crate::models::history::{History, NewHistory};
use crate::schema::histories::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

// Créer une entrée dans l'historique
pub async fn create_history(
    state: web::Data<AppState>,
    new_history: web::Json<NewHistory>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_history = NewHistory {
        water_closet_id: new_history.water_closet_id,
        datetime: new_history.datetime,
        action: new_history.action.clone(),
    };

    match diesel::insert_into(histories)
        .values(&new_history)
        .get_result::<History>(&mut conn)
    {
        Ok(inserted_history) => HttpResponse::Created().json(inserted_history),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert history: {}", err))
        }
    }
}

// Lire toutes les entrées de l'historique
pub async fn get_histories(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match histories.load::<History>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load histories: {}", err))
        }
    }
}

// Lire une entrée spécifique de l'historique
pub async fn get_history(state: web::Data<AppState>, history_id: web::Path<i32>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match histories
        .filter(id.eq(*history_id))
        .first::<History>(&mut conn)
    {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::NotFound().body(format!("History not found: {}", err)),
    }
}

// Mettre à jour une entrée de l'historique
pub async fn update_history(
    state: web::Data<AppState>,
    history_id: web::Path<i32>,
    updated_history: web::Json<NewHistory>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::update(histories.find(*history_id))
        .set((
            water_closet_id.eq(&updated_history.water_closet_id),
            datetime.eq(&updated_history.datetime),
            action.eq(&updated_history.action),
        ))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json(updated_history.into_inner()),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update history: {}", err))
        }
    }
}

// Supprimer une entrée de l'historique
pub async fn delete_history(
    state: web::Data<AppState>,
    history_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match diesel::delete(histories.find(*history_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().body("History deleted"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete history: {}", err))
        }
    }
}
