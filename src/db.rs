
use actix::fut::future::result;
use chess::Game;
use serde::Serialize;
use sqlx::{MySqlPool, MySqlConnection};
use chrono::{DateTime, Utc, TimeZone};
use time::OffsetDateTime;

use crate::models::{User, RegisterUser, LoginUser, UserStatistics, GameSummary};
use crate::game::{GameStatus, calculate_elo_changes};

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
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
        user.username,
        user.email,
        user.password,
    )
    .execute(pool)
    .await?;
    return create_statistics_for_user(&pool, user.elo, &user.email).await;
}


pub async fn create_statistics_for_user(pool: &MySqlPool, elo:i32, email: &str) -> Result<bool, CreateUserError> {
    let user: Option<User> = get_user_by_email(&pool, &email).await?;
    let _result = sqlx::query!(
        "INSERT INTO statistics (user_id, elo, max_elo) VALUES (?, ?, ?)",
        user.unwrap().id,
        elo,
        elo
    )
    .execute(pool)
    .await?;

    Ok(true)
}

pub async fn get_user_by_id(pool: &MySqlPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_statistics(pool: &MySqlPool, user_id: i32) -> Result<Option<UserStatistics>, sqlx::Error> {
    let stats = sqlx::query_as!(
        UserStatistics,
        "SELECT * FROM statistics WHERE user_id = ?",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(stats)
}

pub async fn get_user_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = ?",
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}


pub async fn login_user(pool: &MySqlPool, user: &LoginUser) -> Result<User, LoginUserError> {

    let user: Option<User> = get_user_by_email(pool, &user.email).await?;
    if user.is_none() {
        return Err(LoginUserError::UserDoesNotExist);
    }

    if let Some(user) = user {
        if user.status == "banned" {
            return Err(LoginUserError::UserBanned);
        } else if user.status == "suspended" {
            return Err(LoginUserError::UserSuspended);
        } else if user.status == "active" { 
            return Ok(user);
        }
    }
    
    Err(LoginUserError::UserDoesNotExist)

}

pub async fn create_game(pool: &MySqlPool, white_player_id: i32, black_player_id: i32, white_player_elo: i32, black_player_elo: i32) -> Result<i32, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let result = sqlx::query!(
        "INSERT INTO games (white_player_id, black_player_id, white_player_initial_elo, black_player_initial_elo) VALUES (?, ?, ?, ?)",
        white_player_id,
        black_player_id,
        white_player_elo,
        black_player_elo
    )
    .execute(&mut *conn)
    .await?;

    Ok(result.last_insert_id() as i32)
}

pub async fn add_move_to_game(pool: &MySqlPool, game_id: i32, player_color: &str, move_number: i32, pgn_move: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO moves (game_id, player_color, move_number, pgn_move) VALUES (?, ?, ?, ?)",
        game_id,
        player_color,
        move_number,
        pgn_move
    )
    .execute(pool)
    .await?;

    Ok(())
}



pub async fn end_game(pool: &MySqlPool, game_id: i32, result: &GameStatus) -> Result<(), sqlx::Error> {
    print!("\n\n KONIEC CHUJE \n\n");
    sqlx::query!(
        "UPDATE games SET status = 'finished', result = ?, ended_at = ? WHERE id = ?",
        result.as_str(),
        Utc::now(),
        game_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_games_for_player(pool: &MySqlPool, player_id: i32) -> Result<Vec<GameSummary>, sqlx::Error> {
    let games = sqlx::query_as!(
    GameSummary,
    r#"
    SELECT
        CAST(g.id AS SIGNED) AS game_id,
        g.game_type,
        CASE
            WHEN g.white_player_id = ? THEN 'white'
            ELSE 'black'
        END AS player_color,
        u.username AS username,
        CASE
            WHEN g.white_player_id = ? THEN g.white_player_initial_elo
            ELSE g.black_player_initial_elo
        END AS elo,
        ou.username AS opponent_username,
        CASE
            WHEN g.white_player_id = ? THEN g.black_player_initial_elo
            ELSE g.white_player_initial_elo
        END AS opponent_elo,
        (
            SELECT COUNT(*) FROM moves m WHERE m.game_id = g.id
        ) AS move_count,
        CASE
            WHEN TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at) >= 3600 THEN
                CONCAT(
                    FLOOR(TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at) / 3600), 'h ',
                    FLOOR((TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at) % 3600) / 60), 'm ',
                    TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at) % 60, 's'
                )
            WHEN TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at) >= 60 THEN
                CONCAT(
                    FLOOR(TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at) / 60), 'm ',
                    TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at) % 60, 's'
                )
            ELSE
                CONCAT(
                    TIMESTAMPDIFF(SECOND, g.started_at, g.ended_at), 's'
                )
        END AS duration,
        g.started_at,
        CASE
            WHEN g.result = 'draw' THEN 'draw'
            WHEN (g.result = 'white_win' AND g.white_player_id = ?) OR
                 (g.result = 'black_win' AND g.black_player_id = ?) THEN 'win'
            ELSE 'lose'
        END AS result
    FROM games g
    JOIN users u ON u.id = ?
    JOIN users ou ON (
        ou.id = CASE
            WHEN g.white_player_id = ? THEN g.black_player_id
            ELSE g.white_player_id
        END
    )
    WHERE (g.white_player_id = ? OR g.black_player_id = ?)
      AND g.status = 'finished'
    ORDER BY g.started_at DESC
    "#,
    player_id,
    player_id,
    player_id,
    player_id,
    player_id,
    player_id,
    player_id,
    player_id,
    player_id
)
.fetch_all(pool)
.await?;

    Ok(games)
}


pub async fn update_user_statistics_win(pool: &MySqlPool, winner_id: i32, winner_elo_change: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE statistics SET games_played = games_played + 1, games_won = games_won + 1, current_win_streak = current_win_streak + 1,
        max_win_streak = CASE
            WHEN current_win_streak > max_win_streak THEN current_win_streak
            ELSE max_win_streak
        END,
        elo = CASE
            WHEN elo + ? < 100 THEN 100
            ELSE elo + ?
        END,
        max_elo = CASE
            WHEN elo > max_elo THEN elo 
            ELSE max_elo
        END,
        last_game_at = ? WHERE user_id = ?",
        winner_elo_change,
        winner_elo_change,
        Utc::now(),
        winner_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_user_statistics_lose(pool: &MySqlPool, loser_id: i32, loser_elo_change: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE statistics SET games_played = games_played + 1, games_lost = games_lost + 1, current_win_streak = 0,
        elo = CASE
            WHEN elo + ? < 100 THEN 100
            ELSE elo + ?
        END,
        max_elo = CASE
            WHEN elo > max_elo THEN elo 
            ELSE max_elo
        END,
        last_game_at = ? WHERE user_id = ?",
        loser_elo_change,
        loser_elo_change,
        Utc::now(),
        loser_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_user_statistics_draw(pool: &MySqlPool, white_player_id: i32, black_player_id: i32, white_elo_change: i32, black_elo_change: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE statistics SET games_played = games_played + 1, games_drawn = games_drawn + 1, current_win_streak = 0,
        elo = CASE
            WHEN elo + ? < 100 THEN 100
            ELSE elo + ?
        END,
        max_elo = CASE
            WHEN elo > max_elo THEN elo 
            ELSE max_elo
        END,
        last_game_at = ? WHERE user_id = ?",
        white_elo_change,
        white_elo_change,
        Utc::now(),
        white_player_id
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "UPDATE statistics SET games_played = games_played + 1, games_drawn = games_drawn + 1, current_win_streak = 0,
        elo = CASE
            WHEN elo + ? < 100 THEN 100
            ELSE elo + ?
        END,
        max_elo = CASE
            WHEN elo > max_elo THEN elo 
            ELSE max_elo
        END,
        last_game_at = ? WHERE user_id = ?",
        black_elo_change,
        black_elo_change,
        Utc::now(),
        black_player_id
    )
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn update_users_statistics(pool: &MySqlPool, white_player_id: i32, black_player_id: i32, result: &GameStatus) -> Result<(), sqlx::Error> {
    
    let white_stats = get_user_statistics(pool, white_player_id).await?;
    let black_stats = get_user_statistics(pool, black_player_id).await?;

    let white_elo = white_stats.unwrap().elo;
    let black_elo = black_stats.unwrap().elo;

    let (white_elo_change, black_elo_change) = calculate_elo_changes(white_elo, black_elo, result);
    match result {
        GameStatus::WhiteWin => {
            update_user_statistics_win(pool, white_player_id, white_elo_change).await?;
            update_user_statistics_lose(pool, black_player_id, black_elo_change).await?;
        },
        GameStatus::BlackWin => {
            update_user_statistics_win(pool, black_player_id, black_elo_change).await?;
            update_user_statistics_lose(pool, white_player_id, white_elo_change).await?;
        },
        GameStatus::Draw => {
           update_user_statistics_draw(pool, white_player_id, black_player_id, white_elo_change, black_elo_change).await?;
        },
        _ => {}
    }
    

    Ok(())
}

pub async fn add_chat_message(pool: &MySqlPool, game_id: i32, sender_id: i32, message: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO chat_messages (game_id, sender_id, message) VALUES (?, ?, ?)",
        game_id,
        sender_id,
        message,
    )
    .execute(pool)
    .await?;

    Ok(())
}