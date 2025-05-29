
use sqlx::MySqlPool;
use chrono::{DateTime, Utc, TimeZone};
use time::OffsetDateTime;

use crate::models::{User, RegisterUser, LoginUser};

pub enum CreateUserError {
    UsernameExists,
    EmailExists,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for CreateUserError {
    fn from(e: sqlx::Error) -> Self {
        CreateUserError::DatabaseError(e)
    }
}

pub enum LoginUserError{
    UserDoesNotExist,
    UserBanned,
    UserSuspended,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for LoginUserError {
    fn from(e: sqlx::Error) -> Self {
        LoginUserError::DatabaseError(e)
    }
}


fn convert_opt_offsetdatetime_to_chrono(opt: OffsetDateTime) -> Option<DateTime<Utc>> {
    // let naive = NaiveDateTime::new(
    //     chrono::NaiveDate::from_ymd_opt(opt.year().into(), opt.month() as u32, opt.day().into())?,
    //     chrono::NaiveTime::from_hms_nano_opt(
    //         opt.hour().into(),
    //         opt.minute().into(),
    //         opt.second().into(),
    //         opt.nanosecond().into(),
    //     )?,
    // );

    let timestamp = opt.unix_timestamp();
    let nanos = opt.nanosecond();

    Some(Utc.timestamp_opt(timestamp, nanos).single()?)
}



pub async fn create_user(pool: &MySqlPool, user: &RegisterUser) -> Result<bool, CreateUserError> {
    let email_exists = sqlx::query!(
        "SELECT * FROM users WHERE email = ?",
        user.email
    )
    .fetch_optional(pool)
    .await?;

    if email_exists.is_some() {
        return Err(CreateUserError::EmailExists);
    }

    let username_exists = sqlx::query!(
        "SELECT * FROM users WHERE username = ?",
        user.username
    )
    .fetch_optional(pool)
    .await?;

    if username_exists.is_some() {
        return Err(CreateUserError::UsernameExists);
    }

    let _result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, elo) 
        VALUES (?, ?, ?, ?)",
        user.username,
        user.email,
        user.password,
        user.elo,
    )
    .execute(pool)
    .await?;

    Ok(true)
}


pub async fn login_user(pool: &MySqlPool, user: &LoginUser) -> Result<User, LoginUserError> {
    let user = sqlx::query!(
        "SELECT * FROM users WHERE email = ?",
        user.email
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user {
        if user.status == "banned" {
            return Err(LoginUserError::UserBanned);
        }
        else if user.status == "suspended" {
            return Err(LoginUserError::UserSuspended);
            
        }
        else{
            return Ok(User {
                id: user.id,
                username: user.username,
                email: user.email,
                password_hash: user.password_hash,
                elo: user.elo,
                role: user.role,
                created_at: Some(user.created_at),
            });
        }
    }
    
    Err(LoginUserError::UserDoesNotExist)

}