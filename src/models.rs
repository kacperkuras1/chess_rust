use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;



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
pub struct UserStatistics{
    pub user_id: i32,
    pub games_played: i32,
    pub games_won: i32,
    pub games_lost: i32,
    pub games_drawn: i32,
    pub current_win_streak: i32,
    pub max_win_streak: i32,
    pub elo: i32,
    pub max_elo: i32,
    pub last_game_at: Option<DateTime<Utc>>,
}


#[derive(Deserialize, Serialize, FromRow)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}
