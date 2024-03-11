use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Auth(Auth),
    Client,
    Sys
}

#[derive(Debug)]
pub enum Auth {
    Email,
    Password,
    Session,
    General
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->>> {:<12} - {self:?}", "ERROR");

        return match self {
            Self::Auth(Auth::Email) => (StatusCode::UNAUTHORIZED, Json(json!({
                "success": false,
                "message": "AUTH: Error on email"
            }))).into_response(),
            Self::Auth(Auth::Password) => (StatusCode::UNAUTHORIZED, Json(json!({
                "success": false,
                "message": "AUTH: Error on password"
            }))).into_response(),
            Self::Auth(Auth::Session) => (StatusCode::UNAUTHORIZED, Json(json!({
                "success": false,
                "message": "AUTH: Error on session"
            }))).into_response(),
            Self::Auth(Auth::General) => (StatusCode::UNAUTHORIZED, Json(json!({
                "success": false,
                "message": "AUTH: Error"
            }))).into_response(),
            Self::Client => (StatusCode::BAD_REQUEST, Json(json!({
                "success": false,
                "message": "CLIENT: Bad request"
            }))).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "success": false,
                "message": "UNHANDLED_CLIENT_ERROR"
            })))
            .into_response()
        }
    }
}
