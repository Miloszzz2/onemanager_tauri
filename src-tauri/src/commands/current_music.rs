use tauri::{
    api::process::{Command, CommandEvent},
    Manager,
};

use crate::structs::current_song::CurrentSong;

#[tauri::command]
pub fn current_music(app: tauri::AppHandle) -> Result<(), String> {
    let mut songs: Vec<String> = vec![];
    // Check if the process is already running
    let process_exists = check_process_exists("currentSong3.exe");
    if process_exists {
        println!("Process is already running.");
        return Ok(());
    }

    println!("{}", "robie");
    let (mut rx, _) = Command::new_sidecar("currentSong3")
        .expect("failed to create `` binary command")
        .encoding(encoding_rs::WINDOWS_1250)
        .spawn()
        .expect("Failed to spawn sidecar");
    let window = app.get_window("main").unwrap();

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(message) = event {
                songs.push(message.clone());

                window
                    .emit(
                        "current_song",
                        CurrentSong {
                            message: message.into(),
                        },
                    )
                    .unwrap()
            } else {
                println!("error");
            }
        }
    });
    Ok(())
}

fn check_process_exists(process_name: &str) -> bool {
    use std::process::Command;
    let output = Command::new("tasklist")
        .args(&["/fi", &format!("IMAGENAME eq {}", process_name)])
        .output()
        .expect("Failed to execute tasklist command.");

    let output_str = String::from_utf8_lossy(&output.stdout);
    !output_str.contains("INFO: No tasks are running which match the specified criteria.")
}
