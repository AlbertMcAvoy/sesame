use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use models::database::AppState;
use actix::Actor;
use r2d2::Pool;
use ws::server;
use std::env;
use std::sync::Arc;
use controllers::web_socket;

mod models;
mod schema;
mod controllers;
mod services;
mod ws;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

fn get_pool() -> PostgresPool {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool")
}

fn logging_setup() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging_setup();
    let pool = get_pool();
    let state = Arc::new(AppState { conn: pool });

    // start chat server actor
    let server = server::Server::new().start();

    println!("Backend launched!");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::from(state.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(web_socket::chat_route))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
