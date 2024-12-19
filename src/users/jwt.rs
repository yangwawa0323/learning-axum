use std::time::Duration;

use axum::{http::StatusCode, routing::post, Json, Router};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn create_routes() -> Router {
    Router::new().route("/login", post(fake_login))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    username: String,
    avatar: String,
}

pub const SECRET: &'static str = "this_is_secret";

pub fn create_jwt(username: &str, avatar: &str) -> Result<String, StatusCode> {
    let mut now: DateTime<Utc> = Utc::now();
    let expires = Duration::from_secs(60);
    now += expires;
    let exp = now.timestamp() as usize;
    let claims = Claims {
        exp,
        username: username.to_string(),
        avatar: avatar.to_string(),
    };
    let key = EncodingKey::from_secret(SECRET.as_bytes());
    let token = encode(&Header::default(), &claims, &key).map_err(|_err| {
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;

    Ok(token)
}

pub fn is_valid(token: &str) -> Result<bool, StatusCode> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|err| match err.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .claims;
    tracing::info!("the claims: {claims:#?}");
    Ok(true)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

async fn fake_login(Json(login_form): Json<LoginForm>) -> Result<String, StatusCode> {
    // query the database to found the user
    let avatar = "https://www.flaticon.com/free-icon/man_2202112?term=avatar&page=1&position=1&origin=tag&related_id=2202112";
    if login_form.username == "admin" && login_form.password == "secret" {
        let token = create_jwt(&login_form.username, avatar)?;
        Ok(json!({
            "username" : format!("{}@163.com", &login_form.username),
            "authentication" : format!("Bearer {token}") })
        .to_string())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
