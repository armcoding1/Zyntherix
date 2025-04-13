use crate::models::auth::Claims;
use crate::{models::user::CreateUser, repository::user_repo};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sqlx::PgPool;
use std::env;

fn get_jwt_secret() -> Vec<u8> {
    let secret = env::var("JWT_SECRET_KEY").expect("KEY JWT_SECRET_KEY must be set");
    secret.as_bytes().to_vec()
}

pub fn create_jwt(email: &str) -> String {
    let secret_bytes = get_jwt_secret();
    let audience = env::var("JWT_AUDIENCE").expect("KEY JWT_AUDIENCE must be set");
    let issuer = env::var("JWT_ISSUER").expect("KEY JWT_ISSUER must be set");
    let claims = Claims {
        sub: email.into(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        aud: audience,
        iss: issuer,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_bytes.as_ref()),
    )
    .unwrap()
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_bytes = get_jwt_secret();
    let mut validation = Validation::new(Algorithm::HS256);
    let audience = env::var("JWT_AUDIENCE").expect("KEY JWT_AUDIENCE must be set");
    let issuer = env::var("JWT_ISSUER").expect("KEY JWT_ISSUER must be set");

    validation.set_audience(&[audience]);
    validation.set_issuer(&[issuer]);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_bytes.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}

pub async fn register(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<CreateUser, sqlx::Error> {
    let hash = match hash(password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Err(sqlx::Error::Protocol("Error hashing password".into())),
    };
    user_repo::create_user(pool, email, &hash).await
}

pub async fn login(pool: &PgPool, email: &str, password: &str) -> Option<String> {
    let user = user_repo::find_by_email(pool, email).await.ok()?;
    verify(password, &user.password_hash)
        .ok()
        .filter(|&ok| ok)
        .map(|_| create_jwt(&user.email))
}
