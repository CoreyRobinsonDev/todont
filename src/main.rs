mod models;
mod api;
mod error;

use axum::{Router, routing::{post, get, put}};
use sqlx::PgPool;


#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/todos"
    )] pool: PgPool
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    let mut router = Router::new()
        .route("/api/users/log_in", post(api::user::log_in))
        .route("/api/users/log_out", post(api::user::log_out))
        .route("/api/users", post(api::user::create_account))
        .route("/api/notes", post(api::note::create_note))
        .route("/api/notes", get(api::note::get_notes))
        .route("/api/notes/:id", put(api::note::update_note))
        .with_state(TodontDB { pool })
        .layer(tower_cookies::CookieManagerLayer::new())
        .nest_service("/", tower_http::services::ServeDir::new("frontend"));

    // Live reload the frontend during development
    if cfg!(debug_assertions) {
        router = router.layer(tower_livereload::LiveReloadLayer::new());
    }

    Ok(router.into())
}


#[derive(Clone)]
pub struct TodontDB {
    pool: PgPool
}
