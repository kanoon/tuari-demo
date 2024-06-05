// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod database;
mod ldap;

use std::fs::OpenOptions;
use std::io::Write;

use tauri::State;
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
fn log_message(level: String, message: String) {
  let log_file_path = "app_logs.log"; // Adjust the path as needed

  // Open the log file in append mode, create if it doesn't exist
  let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(log_file_path)
      .expect("Unable to open log file");

  writeln!(file, "[{}] - {}", level, message).expect("Unable to write to log file");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Authentication ...");
    let _ = ldap::check_ldap();

    
    let pool = database::establish_connection().await;
    let app_state = AppState { pool };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![fetch_users,log_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
