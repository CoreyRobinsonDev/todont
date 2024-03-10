use base64::prelude::*;
use pwhash::bcrypt;
use regex::Regex;
use axum::{
    Json, 
    http::StatusCode, response::IntoResponse, extract::State, debug_handler,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tower_cookies::{Cookies, Cookie};
use uuid::Uuid;

use crate::{error::{Result, Error}, api, models::{User, Session}, TodontDB};

pub async fn log_in(
    cookies: Cookies, 
    State(state): State<TodontDB>,
    payload: Json<LoginPayload>, 
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - log_in", "HANDLER");


    let Ok(user) = sqlx::query_as::<_, User>("
        SELECT * FROM t_user
        WHERE email = $1
    ")
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await else { return Err(Error::Login); };

    if !bcrypt::verify(&payload.password, &user.password) {
        return Err(Error::Login); 
    }

    if create_session(user.id, cookies, &state.pool).await.is_none() {
        return Err(Error::Login);
    }

    return Ok((StatusCode::OK, Json(json!({
        "success": true,
        "message": user.id
    }))));
}

pub async fn log_out(
    cookies: Cookies,
    State(state): State<TodontDB>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - log_out", "HANDLER");

    if remove_session(cookies, &state.pool).await.is_none() {
        return Err(Error::Login);
    };

    return Ok((StatusCode::OK, Json(json!({
        "success": true,
        "message": ""
    }))))
}

pub async fn sign_in(
    cookies: Cookies,
    State(state): State<TodontDB>,
    payload: Json<SignInPayload>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - sign_in", "HANDLER");

    let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();

    if payload.password != payload.confirm_password {
        return Err(Error::Login);
    } else if !re.is_match(&payload.email) {
        return Err(Error::Login);
    } else if let Ok(_) = sqlx::query("
        SELECT * FROM t_user
        WHERE email = $1
    ")
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await { return Err(Error::Login) };

    let id = uuid::Uuid::new_v4();

    if create_session(id, cookies, &state.pool).await.is_none() {
        return Err(Error::Login);
    }

    match sqlx::query("
        INSERT INTO t_user
        (id, email, password)
        VALUES ($1, $2, $3)")
        .bind(&id)
        .bind(&payload.email)
        .bind(bcrypt::hash(&payload.password).unwrap())
        .execute(&state.pool)
        .await { 
            Ok(_) => return Ok((StatusCode::CREATED, Json(json!({
                    "success": true,
                    "message": id
                })))),
            Err(e) => return Err(Error::Sys)
        };
}


async fn create_session(
    user_id: Uuid, 
    cookies: Cookies, 
    pool: &PgPool
) -> Option<String> {
    let session: Session;

    if cookies.get(api::AUTH_TOKEN).is_some() {
        return None;
    }

    match sqlx::query_as::<_, Session>("
        INSERT INTO session
        (user_id)
        VALUES ($1) 
        RETURNING id, user_id")
        .bind(&user_id)
        .fetch_one(pool)
        .await {
            Ok(s) =>  session = s,
            Err(_) => return None
        }
    let id = BASE64_STANDARD.encode(session.id.to_be_bytes());
    cookies.add(Cookie::new(api::AUTH_TOKEN, id.clone()));
    
    return Some(id);
}

async fn remove_session(
    cookies: Cookies, 
    pool: &PgPool
) -> Option<i32> {
    let Some(cookie) = cookies.get(api::AUTH_TOKEN) else {
        return None;
    };

    let id: i32 = BASE64_STANDARD.decode(cookie.value())
        .unwrap()
        .iter()
        .map(|num| *num as i32)
        .sum();

    cookies.remove(Cookie::from(api::AUTH_TOKEN));

    match sqlx::query("
        DELETE FROM session
        WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await {
            Ok(_) => return Some(id),
            Err(_) => return None
        }
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct SignInPayload {
    pub email: String,
    pub password: String,
    pub confirm_password: String
}
