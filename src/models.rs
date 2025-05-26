use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Deserialize, Serialize)]
pub struct FlashMessage {
    pub message: String,
    pub message_type: String,
}

impl FlashMessage {
    pub fn success<S: Into<String>>(msg: S) -> Self {
        FlashMessage {
            message_type: "success".into(),
            message: msg.into(),
        }
    }

    pub fn error<S: Into<String>>(msg: S) -> Self {
        FlashMessage {
            message_type: "error".into(),
            message: msg.into(),
        }
    }

    pub fn info<S: Into<String>>(msg: S) -> Self {
        FlashMessage {
            message_type: "info".into(),
            message: msg.into(),
        }
    }

    pub fn warning<S: Into<String>>(msg: S) -> Self {
        FlashMessage {
            message_type: "warning".into(),
            message: msg.into(),
        }
    }
}



#[derive(Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub elo: i32,
}

#[derive(Deserialize, Serialize)]
pub struct LoginUser{
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, Serialize, FromRow)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub elo: i32,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
}
