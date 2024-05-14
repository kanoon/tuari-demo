// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;

use tauri::State;
use tokio::sync::Mutex;
use sqlx::PgPool;
use dotenv::dotenv;

struct AppState {
    pool: PgPool,
}

#[tauri::command]
async fn fetch_users(state: State<'_, AppState>) -> Result<Vec<database::User>, String> {
    let pool = &state.pool;
    match database::get_users(pool).await {
        Ok(users) => Ok(users),
        Err(e) => Err(format!("Failed to fetch users: {}", e)),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = database::establish_connection().await;
    let app_state = AppState { pool };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![fetch_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
