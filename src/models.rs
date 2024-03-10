use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::Utc;


#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: sqlx::types::Uuid,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>
}

#[derive(Deserialize)]
pub struct UserNew {
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>
}

#[derive(Debug, Serialize, FromRow)]
pub struct Session {
    pub id: i32,
    pub user_id: sqlx::types::Uuid
}
