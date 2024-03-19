#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod commands {
    pub mod get_windows_apps;
    pub mod programs;
    mod utils {
        pub mod add_drive_to_file;
        pub mod file_exist;
        pub mod file_name_without_extension;
        pub mod generate_icon;
        pub mod get_app_name_from_path;
        pub mod get_app_path_from_name;
        pub mod replace_double_backslashes;
        pub mod sanitize_path;
    }
}
mod db {}
mod structs {
    pub mod program;
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::commands::programs::getprogrampaths,
            crate::commands::programs::run_program
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
