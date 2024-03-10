use axum::{response::IntoResponse, http::StatusCode};


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Login,
    Sys
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->>> {:<12} - {self:?}", "ERROR");

        return match self {
            Self::Login => (StatusCode::UNAUTHORIZED, "LOGIN_FAIL").into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR")
            .into_response()
        }
    }
}
