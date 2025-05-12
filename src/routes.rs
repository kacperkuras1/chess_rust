use actix_web::{web, HttpResponse, Responder, get, post}; 
use chrono::Utc;
use tera::Tera;
use lazy_static::lazy_static;
use sqlx::MySqlPool;

use crate::db;
use crate::models::{UserForm, User};


lazy_static! {
    static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Błąd ładowania szablonów Tera: {:?}", e);
                std::process::exit(1);
            }
        }
    };
}


#[get("/")]
async fn index_page() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("name", "przejdz do formularz");
    HttpResponse::Ok().body(TEMPLATES.render("index.html", &context).unwrap())
}


#[get("/person")]
async fn form_page() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("name", "formularz do wysłania");
    HttpResponse::Ok().body(TEMPLATES.render("person.html", &context).unwrap())
}

#[get("/chess/{color}")]
async fn chess_page(path: web::Path<String>) -> impl Responder {
    let mut context = tera::Context::new();
    let color = path.into_inner();
    context.insert("color", &color);
    HttpResponse::Ok().body(TEMPLATES.render("chess.html", &context).unwrap())
}


#[post("/form")]
async fn form_handler(form: web::Form<UserForm>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    
    let first_name = form.first_name.clone();
    let last_name = form.last_name.clone();

    let person: Result<Option<User>, sqlx::Error> = db::get_user(&db_pool, &first_name, &last_name).await;


    let mut context = tera::Context::new();
    match person {
        Ok(Some(user)) => {
            context.insert("person", &user);
            if let Some(dt) = user.created_at {
                context.insert("czas", &dt.format("%Y-%m-%d %H:%M:%S").to_string());
                context.insert("data", &dt.date_naive().format("%Y-%m-%d").to_string());
                context.insert("godzina", &dt.time().format("%H:%M:%S").to_string());
                context.insert("jak_dlugo", &Utc::now().signed_duration_since(&dt).num_days());
            }
        }
        Ok(None) => {
            context.insert("error", "Nie znaleziono użytkownika");
        }
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            context.insert("error", "Błąd podczas zapytania do bazy");
        }
    }


    HttpResponse::Ok().body(TEMPLATES.render("person.html", &context).unwrap())
}