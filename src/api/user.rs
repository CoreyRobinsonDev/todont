use axum::{
    Json, 
    http::StatusCode, response::IntoResponse, extract::State,
};
use serde::Deserialize;
use tower_cookies::{Cookies, Cookie};

use crate::{error::{Result, Error}, api, models::User, TodontDB};


pub async fn login(
    cookies: Cookies, 
    State(state): State<TodontDB>,
    payload: Json<LoginPayload>, 
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - login", "HANDLER");

    let Ok(user) = sqlx::query_as::<_, User>("
        SELECT * FROM t_user
        WHERE email = $1
        AND password = $2
    ")
        .bind(&payload.email)
        .bind(&payload.password)
        .fetch_one(&state.pool)
        .await else { return Err(Error::LoginFail); };

    cookies.add(Cookie::new(api::AUTH_TOKEN, "user-1.exp.sign"));

    return Ok((StatusCode::OK, Json(user)));
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
