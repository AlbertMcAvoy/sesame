use crate::models::history::{History, NewHistory};
use crate::schema::histories::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn create_history(
    state: &web::Data<AppState>,
    new_history: &NewHistory,
) -> Result<History, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    let new_history = NewHistory {
        water_closet_id: new_history.water_closet_id,
        datetime: new_history.datetime,
        action: new_history.action.clone(),
    };

    diesel::insert_into(histories)
        .values(&new_history)
        .get_result::<History>(&mut conn)
        .map_err(|err| format!("Failed to insert history: {}", err))
}

pub async fn get_histories(state: &web::Data<AppState>) -> Result<Vec<History>, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    histories
        .load::<History>(&mut conn)
        .map_err(|err| format!("Failed to load histories: {}", err))
}

pub async fn get_history(state: &web::Data<AppState>, history_id: i32) -> Result<History, String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    histories
        .filter(id.eq(history_id))
        .first::<History>(&mut conn)
        .map_err(|err| format!("History not found: {}", err))
}

pub async fn update_history(
    state: &web::Data<AppState>,
    history_id: i32,
    updated_history: &NewHistory,
) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::update(histories.find(history_id))
        .set((
            water_closet_id.eq(&updated_history.water_closet_id),
            datetime.eq(&updated_history.datetime),
            action.eq(&updated_history.action),
        ))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to update history: {}", err))
}

pub async fn delete_history(state: &web::Data<AppState>, history_id: i32) -> Result<(), String> {
    let mut conn = state
        .conn
        .get()
        .map_err(|err| format!("Failed to get a connection from the pool: {}", err))?;

    diesel::delete(histories.find(history_id))
        .execute(&mut conn)
        .map(|_| ())
        .map_err(|err| format!("Failed to delete history: {}", err))
}
