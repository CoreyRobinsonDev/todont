use pwhash::bcrypt;
use regex::Regex;
use axum::{
    Json, 
    http::StatusCode, response::IntoResponse, extract::State,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookies, Cookie};

use crate::{error::{Result, Error}, api, models::User, TodontDB};


pub async fn log_in(
    cookies: Cookies, 
    State(state): State<TodontDB>,
    payload: Json<LoginPayload>, 
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - log_in", "HANDLER");

    let Ok(user) = sqlx::query_as::<_, User>("
        SELECT * FROM t_user
        WHERE email = $1
        AND password = $2
    ")
        .bind(&payload.email)
        .bind(&payload.password)
        .fetch_one(&state.pool)
        .await else { return Err(Error::Login); };

    cookies.add(Cookie::new(api::AUTH_TOKEN, "user-1.exp.sign"));

    return Ok((StatusCode::OK, Json(user)));
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
    }

    if let Ok(_) = sqlx::query("
        SELECT * FROM t_user
        WHERE email = $1
    ")
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await { return Err(Error::Login) };

    let id = uuid::Uuid::new_v4();

    match sqlx::query("
        INSERT INTO t_user
        (id, email, password)
        VALUES ($1, $2, $3)")
        .bind(&id)
        .bind(&payload.email)
        .bind(bcrypt::hash(&payload.password).unwrap())
        .execute(&state.pool)
        .await { 
            Ok(_) => {
                cookies.add(Cookie::new(api::AUTH_TOKEN, bcrypt::hash(id).unwrap()));
                return Ok((StatusCode::CREATED, Json(json!({
                    "success": true,
                    "message": id
                }))))
            }
            Err(e) => {
                println!("{e}");
                return Err(Error::Sys);
            }
        };
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
