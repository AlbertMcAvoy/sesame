use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>; 

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: Pool<ConnectionManager<PgConnection>>,
}

impl AppState {
    pub fn get_conn(&self) -> DatabaseConnection {
        self
            .conn
            .get()
            .expect("Failed to get a connection from the pool.")
    }
}