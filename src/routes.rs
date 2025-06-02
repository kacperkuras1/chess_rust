use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder}; 
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use actix_session::Session;
use tera::Tera;
use lazy_static::lazy_static;
use sqlx::MySqlPool;
use actix_web::FromRequest;

use crate::db;
use crate::models::{RegisterUser, LoginUser, UserStatistics, User};
use crate::auth;

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

pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .insert_header(("Location", location))
        .finish()
}


#[get("/")]
async fn home_page(flash_messages: IncomingFlashMessages, db_pool: web::Data<MySqlPool>, session: Session) -> impl Responder {
    let user = session.get::<i32>("user_id").unwrap();
    if user.is_none() {
        return redirect("/login");
    }

    let mut context = tera::Context::new();

    let user: User = db::get_user_by_id(&db_pool, user.unwrap()).await.unwrap().unwrap();
    let user_statistics: UserStatistics = db::get_user_statistics(&db_pool, user.id).await.unwrap().unwrap();

    context.insert("user", &user);
    context.insert("user_statistics", &user_statistics);

    let messages = flash_messages.iter().map(|msg| {
        match msg.level() {
            actix_web_flash_messages::Level::Error => ("error", msg.content()),
            actix_web_flash_messages::Level::Info => ("info", msg.content()),
            actix_web_flash_messages::Level::Success => ("success", msg.content()),
            actix_web_flash_messages::Level::Warning => ("warning", msg.content()),
            actix_web_flash_messages::Level::Debug => ("debug", msg.content()),
        }
        }).collect::<Vec<_>>();
    context.insert("flash_messages", &messages);
    HttpResponse::Ok().body(TEMPLATES.render("home.html", &context).unwrap())
}

#[get("/login")]
async fn login_page(flash_messages: IncomingFlashMessages, session: Session) -> impl Responder {
    let user_id = session.get::<i32>("user_id").unwrap();
    if user_id.is_some() {
        return redirect("/")
    }
    
    let mut context = tera::Context::new();
    let messages = flash_messages.iter().map(|msg| {
        match msg.level() {
            actix_web_flash_messages::Level::Error => ("error", msg.content()),
            actix_web_flash_messages::Level::Info => ("info", msg.content()),
            actix_web_flash_messages::Level::Success => ("success", msg.content()),
            actix_web_flash_messages::Level::Warning => ("warning", msg.content()),
            actix_web_flash_messages::Level::Debug => ("debug", msg.content()),
        }
    }).collect::<Vec<_>>();
    context.insert("flash_messages", &messages);
    HttpResponse::Ok().body(TEMPLATES.render("login.html", &context).unwrap())
}

#[get("/register")]
async fn register_page(flash_messages: IncomingFlashMessages, session: Session) -> impl Responder {
    let user_id = session.get::<i32>("user_id").unwrap();
    if user_id.is_some() {
        return redirect("/")
    }

    let mut context = tera::Context::new();
    let messages = flash_messages.iter().map(|msg| {
        match msg.level() {
            actix_web_flash_messages::Level::Error => ("error", msg.content()),
            actix_web_flash_messages::Level::Info => ("info", msg.content()),
            actix_web_flash_messages::Level::Success => ("success", msg.content()),
            actix_web_flash_messages::Level::Warning => ("warning", msg.content()),
            actix_web_flash_messages::Level::Debug => ("debug", msg.content()),
        }
    }).collect::<Vec<_>>();
    context.insert("flash_messages", &messages);
    HttpResponse::Ok().body(TEMPLATES.render("register.html", &context).unwrap())
}

#[post("/login")]
async fn login_handler(form: web::Form<LoginUser>, db_pool: web::Data<MySqlPool>, session: Session) -> impl Responder {

    let form = form.into_inner();
    
    let user = db::login_user(&db_pool, &form).await;

    match user {
        Ok(user) => {
            if auth::verify_password(&form.password, &user.password_hash) {
                session.insert("user_id", user.id).unwrap();
                FlashMessage::success("Zalogowano pomyślnie!").send();
                return redirect("/");
            } else {
                FlashMessage::warning("Niepoprawne hasło! Spróbuj ponownie.").send();
                return redirect("/login");
            }
        }
        Err(db::LoginUserError::UserDoesNotExist) => {
            FlashMessage::warning("Nie znaleziono użytkownika o podanym emailu.").send();
            return redirect("/login");
        }

        
        Err(db::LoginUserError::UserBanned) => {
            FlashMessage::warning("Twoje konto zostało zbanowane.").send();
            return redirect("/login");
        }
        Err(db::LoginUserError::UserSuspended) => {
            FlashMessage::warning("Twoje konto zostało zawieszone.").send();
            return redirect("/login");
        }
        Err(_) => {
            FlashMessage::error("Wystąpił błąd podczas logowania. Spróbuj ponownie później.").send();
            return redirect("/login");
        }
    }
}

#[post("/register")]
async fn register_handler(form: web::Form<RegisterUser>, db_pool: web::Data<MySqlPool>, session: Session) -> impl Responder {

    let user_id = session.get::<i32>("user_id").unwrap();
    if user_id.is_some() {
        return redirect("/")
    }

    let mut user = RegisterUser{
        username: form.username.clone(),
        email: form.email.clone(),
        password: form.password.clone(),
        password_confirmation: form.password_confirmation.clone(),
        elo: form.elo,
    };

    if ![600, 800, 1000].contains(&user.elo) {
        user.elo = 600;
    }

    if user.password != user.password_confirmation {
        FlashMessage::warning("Hasła nie są takie same!").send();
        return redirect("/register");
    }

    user.password = auth::hash_password(&user.password).unwrap();

    match db::create_user(&db_pool, &user).await{
        Ok(_) =>{
            FlashMessage::success("Rejestracja zakończona sukcesem! Możesz teraz się zalogować.").send();
            return redirect("/login");
        }
        Err(db::CreateUserError::EmailExists) =>{
            FlashMessage::warning("Podany Email jest zajęty.").send();
            return redirect("/register");
        }
        Err(db::CreateUserError::UsernameExists) =>{
            FlashMessage::warning("Podana nazwa użytkownika jest już zajęta.").send();
            return redirect("/register");
        }
        Err(_) => {
            FlashMessage::error("Wystąpił błąd podczas rejestracji. Spróbuj ponownie później.").send();
            return redirect("/register");
        }
    }

}

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.remove("user_id");
    FlashMessage::info("Zostałeś wylogowany.").send();
    redirect("/")
}


#[get("/get_jwt")]
async fn get_jwt(session: Session) -> impl Responder {
    if let Ok(Some(user_id)) = session.get::<i32>("user_id") {
        match auth::create_jwt(user_id) {
            Ok(token) => return HttpResponse::Ok().json(serde_json::json!({ "token": token })),
            Err(_) => return HttpResponse::InternalServerError().body("Błąd JWT"),
        }
    }

    HttpResponse::Unauthorized().body("Nie jesteś zalogowany")
}

#[get("/chess")]
async fn chess_page(flash_messages: IncomingFlashMessages, session: Session) -> impl Responder {
    
    
    print!("\n\n\nDHFJKDHFJKHDJKFHDJKFHDJKFHJKDHFJKDF\n\n\n");

    let user_id = session.get::<i32>("user_id").unwrap();
    if user_id.is_none() {
        return redirect("/")
    }
    let mut context = tera::Context::new();
    let messages = flash_messages.iter().map(|msg| {
        match msg.level() {
            actix_web_flash_messages::Level::Error => ("error", msg.content()),
            actix_web_flash_messages::Level::Info => ("info", msg.content()),
            actix_web_flash_messages::Level::Success => ("success", msg.content()),
            actix_web_flash_messages::Level::Warning => ("warning", msg.content()),
            actix_web_flash_messages::Level::Debug => ("debug", msg.content()),
        }
    }).collect::<Vec<_>>();
    context.insert("flash_messages", &messages);
    HttpResponse::Ok().body(TEMPLATES.render("chess.html", &context).unwrap())
}
