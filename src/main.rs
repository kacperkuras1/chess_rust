use actix_web_flash_messages::{FlashMessagesFramework, storage::CookieMessageStore};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::{Key, SameSite};
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
mod game;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));



    let game_state = web::Data::new(Arc::new(Mutex::new(game::GameState::default())));
    let secret_key = Key::generate();
    let secret_key_data = web::Data::new(secret_key.clone());
    let flash_messages = FlashMessagesFramework::builder(CookieMessageStore::builder(secret_key).build())
        .build();
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
            .wrap(flash_messages.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key_data.get_ref().clone())
                    .cookie_secure(false) // jeśli lokalnie
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(game_state.clone())
            .service(Files::new("/static", "./static").show_files_listing())
            .service(routes::home_page)
            .service(routes::play_page)
            .service(ws::websocket_handler)
            .service(routes::login_page)
            .service(routes::login_handler)
            .service(routes::register_page)
            .service(routes::register_handler)
            .service(routes::logout)
            .service(routes::get_jwt)
            .service(routes::games_page)
            .service(routes::statistics_page)
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}