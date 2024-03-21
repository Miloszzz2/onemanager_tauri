#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use structs::current_song::CurrentSong;
use tauri::{
    api::process::{Command, CommandEvent},
    Manager,
};

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
        pub mod replace_double_backslashes;
        pub mod sanitize_path;
    }
}
mod db {}
mod structs {
    pub mod current_song;
    pub mod program;
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::commands::programs::getprogrampaths,
            crate::commands::programs::run_program
        ])
        .setup(|app| {
            let (mut rx, _) = Command::new_sidecar("currentSong2")
                .expect("failed to create `` binary command")
                .spawn()
                .expect("Failed to spawn sidecar");
            let window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(message) = event {
                        window
                            .emit(
                                "current_song",
                                CurrentSong {
                                    message: message.into(),
                                },
                            )
                            .unwrap()
                    } else {
                        println!("Failed to execute sidecar");
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
