use axum::http::StatusCode;
use axum::{extract::State, Json, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::prelude::FromRow;
use tower_cookies::Cookies;

use crate::TodontDB;
use crate::error::{Result, Error, Auth};
use crate::models::Note;

use super::user::get_user;


pub async fn create_note(
    cookies: Cookies,
    State(state): State<TodontDB>,
    payload: Json<NotePayload>, 
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - create_note", "HANDLER");

    let Some(user) = get_user(&cookies, &state.pool).await else {
        return Err(Error::Auth(Auth::Session));
    };

    let Ok(row) = sqlx::query_as::<_, NoteId>("
        INSERT INTO note
        (user_id, title, description, completed)
        VALUES ($1, $2, $3, $4)
        RETURNING id")
        .bind(&user.id)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(&payload.completed)
        .fetch_one(&state.pool)
        .await else {
            return Err(Error::Sys);
        };

    return Ok((StatusCode::CREATED, Json(json!({
        "success": true,
        "message": row.id.to_string()
    }))))
}

pub async fn get_notes(
    cookies: Cookies,
    State(state): State<TodontDB>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - get_notes", "HANDLER");

    let Some(user) = get_user(&cookies, &state.pool).await else {
        return Err(Error::Auth(Auth::Session));
    };

    let Ok(notes) = sqlx::query_as::<_, Note>("
        SELECT * FROM note
        WHERE user_id = $1")
        .bind(&user.id)
        .fetch_all(&state.pool)
        .await else {
            return Err(Error::Sys);
        };

    return Ok((StatusCode::CREATED, Json(json!({
        "success": true,
        "message": notes
    }))))
}

#[derive(Debug, FromRow)]
struct NoteId {
    id: i32
}

#[derive(Debug, Deserialize)]
pub struct NotePayload {
     title: String,
     description: Option<String>,
     completed: Option<bool>,
}
