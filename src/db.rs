use sqlx::MySqlPool;
use time::PrimitiveDateTime;
use chrono::{NaiveDateTime, DateTime, Utc, TimeZone};


use crate::models::User;

fn convert_opt_primitive_to_chrono(opt: Option<PrimitiveDateTime>) -> Option<DateTime<Utc>> {
    opt.and_then(|primitive| {
        let date = primitive.date();
        let time = primitive.time();

        let naive = NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(date.year(), date.month() as u32, date.day() as u32)?,
            chrono::NaiveTime::from_hms_nano_opt(
                time.hour() as u32,
                time.minute() as u32,
                time.second() as u32,
                time.nanosecond(),
            )?,
        );

        Some(Utc.from_utc_datetime(&naive))
    })
}


pub async fn get_user(pool: &MySqlPool, first_name: &String, last_name: &String) -> Result<Option<User>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT id, first_name, last_name, email, role, password_hash, created_at, updated_at 
        FROM users 
        WHERE first_name = ? AND last_name = ?", 
        first_name, 
        last_name
    )
    .fetch_optional(pool)
    .await?;
    
    let user = result.map(|user| User {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        password_hash: user.password_hash,
        role: user.role,
        created_at: convert_opt_primitive_to_chrono(user.created_at),
        updated_at: convert_opt_primitive_to_chrono(user.updated_at),
    });

    Ok(user)
}
