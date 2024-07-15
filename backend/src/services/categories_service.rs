
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use r2d2::PooledConnection;
use uuid::Uuid;

use crate::{models::categories::{Category, NewCategory}, schema::categories::dsl::{categories, name}};

pub async fn create_category(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_category: NewCategory,
) -> Result<Category, Error> {

    diesel::insert_into(categories)
        .values(&new_category)
        .get_result::<Category>(conn)
}

pub async fn get_categories(conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<Vec<Category>, Error> {
    categories.load::<Category>(conn)
}

pub async fn update_category(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_category: NewCategory,
    id_category: Uuid
) -> Result<Category, Error> {

    diesel::update(categories.find(id_category))
        .set((
            name.eq(&new_category.name),
        ))
        .get_result(conn)
}

pub async fn delete_category(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, id_category: Uuid) -> Result<Category, Error> {
    diesel::delete(categories.find(id_category)).get_result(conn)
}
