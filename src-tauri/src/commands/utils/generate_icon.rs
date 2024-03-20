use std::process::{Command, Output};

use super::file_name_without_extension::file_name_without_extension;

pub fn generate_icon(display_icon_without_prefix: &str) -> Output {
    println!("{}", display_icon_without_prefix.replace("\\\\", "/"));
    let tauri_cmd = tauri::api::process::Command::new_sidecar("iconExtract").unwrap();
    let std_cmd = Command::from(tauri_cmd)
        .args(&[
            &display_icon_without_prefix.replace("\\\\", "/"),
            &format!(
                "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons/{}.png",
                file_name_without_extension(display_icon_without_prefix),
            ),
        ])
        .output()
        .expect("Failed to execute command");

    return std_cmd;
}
