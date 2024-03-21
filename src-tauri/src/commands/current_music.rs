use tauri::{
    api::process::{Command, CommandEvent},
    Manager,
};

use crate::structs::current_song::CurrentSong;
#[tauri::command]
pub fn current_music(app: tauri::AppHandle) -> Result<(), String> {
    println!("{}", "robie");
    let (mut rx, _) = Command::new_sidecar("currentSong3")
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
}
