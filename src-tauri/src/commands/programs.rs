use std::env;
use std::path::Path;
use std::process::{Command, Output};
use walkdir::WalkDir;
#[derive(Clone, serde::Serialize)]
pub struct Program {
    path: String,
    visible: bool,
    name: String,
}

#[tauri::command]
pub(crate) fn getprogrampaths() -> Result<Vec<Program>, String> {
    let apps = installed::list().map_err(|err| err.to_string())?;
    let mut options: Vec<Program> = vec![];

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
                let new_program: Program = Program {
                    path: replace_double_backslashes(&display_icon_without_prefix.to_string()),
                    visible: true,
                    name: get_app_name_from_path(&replace_double_backslashes(
                        &display_icon_without_prefix.to_string(),
                    )),
                };
                if !file_exists(pathtocheck) {
                    let output = generate_icon(&display_icon_without_prefix);
                    if output.status.success() {
                        options.push(new_program);
                    }
                } else {
                    options.push(new_program);
                }
            }
        }
    }
    for entry in WalkDir::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs")
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

                    if options
                        .iter()
                        .all(|s| s.path != display_icon_without_prefix)
                    {
                        let pathtocheck = format!(
                            "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons/{}.png",
                            file_name_without_extension(&display_icon_without_prefix)
                        );
                        let new_program: Program = Program {
                            path: replace_double_backslashes(
                                &display_icon_without_prefix.to_string(),
                            ),
                            visible: true,
                            name: get_app_name_from_path(&replace_double_backslashes(
                                &display_icon_without_prefix.to_string(),
                            )),
                        };
                        if file_exists(display_icon_without_prefix.clone()) {
                            if !file_exists(pathtocheck) {
                                let output = generate_icon(&display_icon_without_prefix);
                                if output.status.success() {
                                    options.push(new_program);
                                }
                            } else {
                                options.push(new_program);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut appdata = String::new(); // Default value

    if let Ok(path) = env::var("APPDATA") {
        appdata = path;
    } else {
        println!("Environment variable %APPDATA% not found.");
    }
    for entry in WalkDir::new(format!(
        "{}\\Microsoft\\Windows\\Start Menu\\Programs",
        appdata
    ))
    .into_iter()
    .filter_map(|e| e.ok())
    {
        let path = entry.path().to_string_lossy();

        if path.contains(".lnk") && !path.contains("PowerShell") {
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

                    if options
                        .iter()
                        .all(|s| s.path != display_icon_without_prefix)
                    {
                        let pathtocheck = format!(
                            "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons/{}.png",
                            file_name_without_extension(&display_icon_without_prefix)
                        );

                        if file_exists(display_icon_without_prefix.clone()) {
                            let new_program: Program = Program {
                                path: replace_double_backslashes(
                                    &display_icon_without_prefix.to_string(),
                                ),
                                visible: true,
                                name: get_app_name_from_path(&replace_double_backslashes(
                                    &display_icon_without_prefix.to_string(),
                                )),
                            };
                            if !file_exists(pathtocheck) {
                                let output = generate_icon(&display_icon_without_prefix);
                                if output.status.success() {
                                    options.push(new_program);
                                }
                            } else {
                                options.push(new_program);
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
    let mut localappdata = String::new();
    if let Ok(path) = env::var("LOCALAPPDATA") {
        localappdata = path;
    } else {
        println!("Environment variable %APPDATA% not found.");
    }
    if let Some(index) = s.rfind("..\\") {
        let mut full_path = if s.contains("Local") {
            localappdata.to_owned()
        } else {
            "C:\\".to_owned()
        };
        full_path.push_str(&s[index + 3..]);
        // Move index 4 positions to exclude "..//"
        return full_path.replace("\\Local", "\\");
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
