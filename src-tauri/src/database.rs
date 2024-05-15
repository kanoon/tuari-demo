// src-tauri/src/database.rs
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use serde::Serialize;
use std::env;
use dotenv::dotenv;

#[derive(Serialize)]
pub struct User {
    id: i64,
    name: Option<String>,
    email: Option<String>,
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url =  env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
