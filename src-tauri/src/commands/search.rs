use std::process::Command;

#[tauri::command]
pub fn search(url: String) -> Result<(), String> {
    Command::new("cmd")
        .args(&["/C", &format!("start chrome {}", url)])
        .spawn()
        .map_err(|err| err.to_string())?;
    Ok(())
}
