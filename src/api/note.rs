use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::prelude::FromRow;
use sqlx::types::chrono;
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

    let Ok(id) = sqlx::query_as::<_, NoteId>("
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
        "message": id.0.to_string()
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

pub async fn get_note(
    cookies: Cookies,
    State(state): State<TodontDB>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - get_note", "HANDLER");

    let Some(user) = get_user(&cookies, &state.pool).await else {
        return Err(Error::Auth(Auth::Session));
    };


    let Ok(note) = sqlx::query_as::<_,Note>("
        SELECT * FROM note
        WHERE user_id = $1
        AND id = $2")
        .bind(&user.id)
        .bind(&id)
        .fetch_one(&state.pool)
        .await else {
            return Err(Error::Client);
        };

    return Ok((StatusCode::CREATED, Json(json!({
        "success": true,
        "message": note
    }))))
}

pub async fn update_note(
    cookies: Cookies,
    State(state): State<TodontDB>,
    Path(id): Path<i32>,
    payload: Json<NotePayload> 
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_note", "HANDLER");

    let Some(user) = get_user(&cookies, &state.pool).await else {
        return Err(Error::Auth(Auth::Session));
    };

    let Ok(_) = sqlx::query("
        UPDATE note
        SET title = $1,
        description = $2,
        updated_at = $3
        WHERE user_id = $4
        AND id = $5
        RETURNING id")
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(chrono::Utc::now())
        .bind(&user.id)
        .bind(&id)
        .fetch_one(&state.pool)
        .await else {
            return Err(Error::Client);
        };

    return Ok((StatusCode::CREATED, Json(json!({
        "success": true,
        "message": id.to_string()
    }))))

}

#[derive(Debug, FromRow, Deserialize)]
struct NoteId(i32);

#[derive(Debug, Deserialize)]
pub struct NotePayload {
    title: String,
    description: Option<String>,
    completed: Option<bool>,
}
