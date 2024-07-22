use crate::models::database::DatabaseConnection;
use crate::models::history::{History, NewHistory};
use crate::schema::histories::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_history(
    conn: &mut DatabaseConnection,
    new_history: &NewHistory,
) -> Result<History, Error> {

    let new_history = NewHistory {
        water_closet_id: new_history.water_closet_id,
        datetime: new_history.datetime,
        action: new_history.action.clone(),
    };

    diesel::insert_into(histories)
        .values(&new_history)
        .get_result::<History>(conn)
}

pub async fn get_histories(conn: &mut DatabaseConnection) -> Result<Vec<History>, Error> {
    histories.load::<History>(conn)
}

pub async fn get_history(conn: &mut DatabaseConnection, history_id: i32) -> Result<History, Error> {
    histories
        .filter(id.eq(history_id))
        .first::<History>(conn)
}

pub async fn update_history(
    conn: &mut DatabaseConnection,
    history_id: i32,
    updated_history: &NewHistory,
) -> Result<History, Error> {
    diesel::update(histories.find(history_id))
        .set((
            water_closet_id.eq(&updated_history.water_closet_id),
            datetime.eq(&updated_history.datetime),
            action.eq(&updated_history.action),
        ))
        .get_result(conn)
}

pub async fn delete_history(conn: &mut DatabaseConnection, history_id: i32) -> Result<History, Error> {
    diesel::delete(histories.find(history_id))
        .get_result(conn)
}
