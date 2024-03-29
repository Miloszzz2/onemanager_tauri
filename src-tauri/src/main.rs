#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use std::env;

use db::migrations::get_migrations;
#[cfg(target_os = "windows")]
mod commands {
    pub mod current_music;
    pub mod get_windows_apps;
    pub mod programs;
    mod utils {
        pub mod add_drive_to_file;
        pub mod file_exist;
        pub mod file_name_without_extension;
        pub mod generate_icon;
        pub mod get_app_name_from_path;
        pub mod replace_double_backslashes;
        pub mod sanitize_path;
    }
}
mod db {
    pub mod migrations;
}
mod structs {
    pub mod current_song;
    pub mod program;
}
fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(&database_url, get_migrations())
                .build(),
        )
        .plugin(tauri_plugin_oauth::init())
        .invoke_handler(tauri::generate_handler![
            crate::commands::programs::getprogrampaths,
            crate::commands::programs::run_program,
            crate::commands::current_music::current_music,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
