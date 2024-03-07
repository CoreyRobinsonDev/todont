mod routes;

use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::routes::*;


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
        .route("/", get(hello_world))
        .with_state(TodontDB { pool })
        .nest_service("/", tower_http::services::ServeDir::new("frontend"));

    // Live reload the frontend during development
    if cfg!(debug_assertions) {
        router = router.layer(tower_livereload::LiveReloadLayer::new());
    }

    Ok(router.into())
}

#[derive(Clone)]
struct TodontDB {
    pool: PgPool
}
