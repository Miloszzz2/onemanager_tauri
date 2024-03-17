use super::structs::program::Program;
use super::utils::file_exist::file_exists;
use super::utils::file_name_without_extension::file_name_without_extension;
use super::utils::generate_icon::generate_icon;

fn get_app_name_from_path(app: &str) -> String {
    if let Some(last_index) = app.rfind('\\') {
        let app_name = &app[last_index + 1..app.len() - 4];
        let first_char = app_name
            .chars()
            .next()
            .unwrap_or('_')
            .to_uppercase()
            .to_string();
        let rest_of_name = &app_name[1..];
        format!("{}{}", first_char, rest_of_name)
    } else {
        String::new()
    }
}
pub(crate) fn get_windows_programs() -> Vec<Program> {
    let windows_programs_paths: Vec<&str> = vec!["C:\\Windows\\System32\\wsl.exe", "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Administrative Tools\\dfrgui.lnk", "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Administrative Tools\\Disk Cleanup.lnk", "C:\\Windows\\System32\\control.exe", "C:\\Windows\\System32\\calc.exe", "C:\\Windows\\notepad.exe", "C:\\Program Files\\Windows NT\\Accessories\\wordpad.exe"];
    let mut windows_programs: Vec<Program> = Vec::new();
    for path in &windows_programs_paths {
        let new_program = Program {
            path: path.to_string(),
            visible: true,
            name: get_app_name_from_path(&path),
        };
        let pathtocheck = format!(
            "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons/{}.png",
            file_name_without_extension(&path)
        );

        if file_exists(path.to_string().clone()) {
            if !file_exists(pathtocheck) {
                let output = generate_icon(&path);
                if output.status.success() {
                    windows_programs.push(new_program);
                }
            } else {
                windows_programs.push(new_program);
            }
        }
    }

    return windows_programs;
}
