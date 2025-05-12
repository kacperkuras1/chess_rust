use actix_web::{App, HttpServer, web};
use sqlx::mysql::MySqlPoolOptions;
use actix_web::middleware::Logger;
use std::sync::{Arc, Mutex};
use actix_files::Files;
use dotenv::dotenv;
use std::env;

use env_logger::Env;

mod db;
mod models;
mod routes;
mod ws;
mod auth;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let game_state = web::Data::new(Arc::new(Mutex::new(ws::GameState::default())));



    let addr = env::var("ADDRESS").expect("ADDRESS not set!");
    let port = env::var("PORT").expect("PORT not set!");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(game_state.clone())
            .service(Files::new("/static", "./static").show_files_listing())
            .service(routes::index_page)
            .service(routes::form_page)
            .service(routes::form_handler)
            .service(routes::chess_page)
            .service(ws::websocket_handler)
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}
