#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands {
    pub(crate) mod programs;
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                crate::commands::programs::getprogrampaths,
                crate::commands::programs::runprogram
            ]
        )

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
