use std::path::Path;
use std::process::{Command, Output};
use walkdir::WalkDir;
#[tauri::command]
pub(crate) fn getprogrampaths() -> Result<Vec<String>, String> {
    let apps = installed::list().map_err(|err| err.to_string())?;
    let mut options = vec![];

    for app in apps {
        let x = app.dump();
        if let Some(display_icon) = x.lines().find(|line| {
            line.starts_with("DisplayIcon")
                && line.contains(".exe")
                && !line.contains("uninstall")
                && !line.contains("Uninstall")
                && !line.contains("unins")
                && !line.contains("Installer")
                && !line.contains("setup")
                && !line.contains("{")
        }) {
            let display_icon_without_prefix = sanitize_path(
                &display_icon
                    .trim_start_matches("DisplayIcon:")
                    .trim()
                    .replace("\"", ""),
            );
            let pathtocheck = format!(
                "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons/{}.png",
                file_name_without_extension(&display_icon_without_prefix)
            );

            if file_exists(display_icon_without_prefix.clone()) {
                if !file_exists(pathtocheck) {
                    let output = generate_icon(&display_icon_without_prefix);
                    if output.status.success() {
                        options.push(replace_double_backslashes(
                            &display_icon_without_prefix.to_string(),
                        ));
                    }
                } else {
                    options.push(replace_double_backslashes(
                        &display_icon_without_prefix.to_string(),
                    ));
                }
            }
        }
    }
    for entry in WalkDir::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_string_lossy();
        if path.contains(".lnk") {
            let mut good_path = entry.path().to_string_lossy().to_mut().to_string();
            good_path.truncate(good_path.len() - 4);

            let shortcut = lnk::ShellLink::open(entry.path()).unwrap();

            if let Some(relative) = shortcut.relative_path() {
                if relative.contains(".exe")
                    && !relative.contains("uninstall")
                    && !relative.contains("Uninstall")
                    && !relative.contains("unins")
                    && !relative.contains("Installer")
                    && !relative.contains("setup")
                    && !relative.contains("{")
                    && !relative.contains("dll")
                {
                    let display_icon_without_prefix = add_drive_to_path(&relative);
                    if !options.contains(&display_icon_without_prefix) {
                        let pathtocheck = format!(
                            "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons/{}.png",
                            file_name_without_extension(&display_icon_without_prefix)
                        );

                        if file_exists(display_icon_without_prefix.clone()) {
                            if !file_exists(pathtocheck) {
                                let output = generate_icon(&display_icon_without_prefix);
                                if output.status.success() {
                                    options.push(display_icon_without_prefix.to_string());
                                }
                            } else {
                                options.push(display_icon_without_prefix.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(options)
}

fn generate_icon(display_icon_without_prefix: &str) -> Output {
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

fn replace_double_backslashes(s: &str) -> String {
    s.replace("\\\\", "\\")
}
fn add_drive_to_path(s: &str) -> String {
    if let Some(index) = s.rfind("..\\") {
        // If "..//" is found, prepend "C:/" to the string
        let mut full_path = "C:\\".to_owned();
        full_path.push_str(&s[index + 3..]); // Move index 4 positions to exclude "..//"
        return full_path;
    }
    // If "..//" is not found, return the original string
    s.to_string()
}

fn file_exists(file_path: String) -> bool {
    let path = Path::new(&file_path);
    path.exists() && path.is_file()
}

fn file_name_without_extension(path: &str) -> String {
    let file_name = Path::new(path)
        .file_stem()
        .expect("Invalid path")
        .to_string_lossy()
        .to_string();

    let mut file_name_without_extension = file_name.to_owned();

    if let Some(first_char) = file_name_without_extension.get_mut(0..1) {
        first_char.make_ascii_uppercase();
    }

    file_name_without_extension
}

fn sanitize_path(path: &str) -> String {
    if let Some(index) = path.find(',') {
        path[..index].to_string()
    } else {
        path.to_string()
    }
}
#[tauri::command]
pub(crate) fn run_program(path: String) -> Result<(), String> {
    if path.contains("powershell")
        || path.contains("node")
        || (path.contains("cmd.exe") && path.contains("System"))
    {
        println!("{}", file_name_without_extension(&path));
        Command::new("sh")
            .current_dir("C:/Users/")
            .args(&[
                "-c",
                &format!("start {}", file_name_without_extension(&path)),
            ])
            .spawn()
            .map_err(|err| err.to_string())?;
    } else {
        println!("{}", path);
        Command::new(path).spawn().map_err(|err| err.to_string())?;
    }
    Ok(())
}
