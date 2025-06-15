use sqlx::{Pool, Sqlite, SqlitePool};
use std::{env, fs};
use tauri::{AppHandle, Manager};

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Self {
        let app_dir = app_handle.path().app_data_dir().unwrap();

        // Ensure the app directory exists
        let _ = fs::create_dir_all(&app_dir);

        let db_path = app_dir.join("iracehud.db");

        // Set the DATABASE_URL environment variable to point to this SQLite file
        unsafe { env::set_var("DATABASE_URL", format!("sqlite://{}", db_path.display())) };

        let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(connection_options).await.unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        Self { pool }
    }
}

// State management for Tauri
#[allow(dead_code)]
pub struct DatabaseState(pub Pool<Sqlite>);
