use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use controllers::auth_controller::check_mail;
use controllers::group_controller::{
    create_group, delete_group, get_group, get_groups, update_group,
};
use controllers::history_controller::{
    create_history, delete_history, get_histories, get_history, update_history,
};
use controllers::place_controller::{
    create_place, delete_place, get_place, get_places, update_place,
};
use controllers::report_controller::{
    create_report, delete_report, get_report, get_reports, update_report,
};
use controllers::user_controller::{create_user, delete_user, get_user, get_users, update_user};
use controllers::water_closet_controller::{
    create_water_closet, delete_water_closet, get_water_closet, get_water_closets,
    update_water_closet,
};
use controllers::web_socket;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use models::database::AppState;
use r2d2::Pool;
use std::env;
use std::sync::Arc;
use ws::server;

mod controllers;
mod models;
mod schema;
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

    // start server actor
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
            .route("/ws", web::get().to(web_socket::ws_route))
            .service(web::scope("/auth").route("", web::post().to(check_mail)))
            .service(
                web::scope("/groups")
                    .route("", web::post().to(create_group))
                    .route("", web::get().to(get_groups))
                    .route("/{id}", web::get().to(get_group))
                    .route("/{id}", web::put().to(update_group))
                    .route("/{id}", web::delete().to(delete_group)),
                /*.route("/{post_id}/comments", web::post().to(create_post)), */
            )
            .service(
                web::scope("/histories")
                    .route("", web::post().to(create_history))
                    .route("", web::get().to(get_histories))
                    .route("/{id}", web::get().to(get_history))
                    .route("/{id}", web::put().to(update_history))
                    .route("/{id}", web::delete().to(delete_history)),
            )
            .service(
                web::scope("/places")
                    .route("", web::post().to(create_place))
                    .route("", web::get().to(get_places))
                    .route("/{id}", web::get().to(get_place))
                    .route("/{id}", web::put().to(update_place))
                    .route("/{id}", web::delete().to(delete_place)),
            )
            .service(
                web::scope("/reports")
                    .route("", web::post().to(create_report))
                    .route("", web::get().to(get_reports))
                    .route("/{id}", web::get().to(get_report))
                    .route("/{id}", web::put().to(update_report))
                    .route("/{id}", web::delete().to(delete_report)),
            )
            .service(
                web::scope("/users")
                    .route("", web::post().to(create_user))
                    .route("", web::get().to(get_users))
                    .route("/{id}", web::get().to(get_user))
                    .route("/{id}", web::put().to(update_user))
                    .route("/{id}", web::delete().to(delete_user)),
            )
            .service(
                web::scope("/waterclosets")
                    .route("", web::post().to(create_water_closet))
                    .route("", web::get().to(get_water_closets))
                    .route("/{id}", web::get().to(get_water_closet))
                    .route("/{id}", web::put().to(update_water_closet))
                    .route("/{id}", web::delete().to(delete_water_closet)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
