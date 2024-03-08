use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::Utc;


#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: String,
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
