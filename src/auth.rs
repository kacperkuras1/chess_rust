use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, PasswordHash, SaltString, PasswordVerifier}, Argon2,};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData, errors::Result as JwtResult};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

const SECRET_KEY: &[u8] = b"mega_super_secret_key_1234567890";




#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,          
    pub exp: usize,        
}

pub fn create_jwt(user_id: i32) -> JwtResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}

pub fn verify_jwt(token: &str) -> JwtResult<TokenData<Claims>> {
    match decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &Validation::default()) {
        Ok(data) => Ok(data),
        Err(e) => {
            println!("JWT verification error: {:?}", e);
            Err(e)
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}