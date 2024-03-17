use super::file_name_without_extension::file_name_without_extension;
use std::process::{Command, Output};
pub fn generate_icon(display_icon_without_prefix: &str) -> Output {
    let args = [
        format!("'{}'", display_icon_without_prefix),
        format!(
            "'{}'",
            file_name_without_extension(&display_icon_without_prefix)
        ),
        format!(
            "'{}'",
            "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons"
        ),
    ];

    let python_command_with_args = format!(
        "source venv/Scripts/activate && python main.py {}",
        args.join(" ")
    );

    let output = Command::new("sh")
        .current_dir("./iconExtract")
        .arg("-c")
        .arg(python_command_with_args)
        .output()
        .expect("Failed to execute command");
    return output;
}
