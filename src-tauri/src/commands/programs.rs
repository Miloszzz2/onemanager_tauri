use super::get_windows_apps::get_windows_programs;
use super::utils::add_drive_to_file::add_drive_to_path;
use super::utils::file_exist::file_exists;
use super::utils::file_name_without_extension::file_name_without_extension;
use super::utils::generate_icon::generate_icon;
use super::utils::get_app_name_from_path::get_app_name_from_path;
use super::utils::replace_double_backslashes::replace_double_backslashes;
use super::utils::sanitize_path::sanitize_path;
use crate::structs::program::Program;
use std::env;
use std::process::Command;
use walkdir::WalkDir;
pub fn getprogrampaths_fn() -> Vec<Program> {
    let apps = installed::list().map_err(|err| err.to_string()).unwrap();
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

                    if !options.iter().any(|s| {
                        s.name
                            == get_app_name_from_path(&replace_double_backslashes(
                                &display_icon_without_prefix.to_string(),
                            ))
                    }) {
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

                    if !options.iter().any(|s| {
                        s.name
                            == get_app_name_from_path(&replace_double_backslashes(
                                &display_icon_without_prefix.to_string(),
                            ))
                    }) {
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
    let windows_programs: Vec<Program> = get_windows_programs();
    options.extend(windows_programs);
    return options;
}
#[tauri::command]
pub fn getprogrampaths() -> Result<Vec<Program>, String> {
    let program_paths = getprogrampaths_fn();
    Ok(program_paths)
}

#[tauri::command]
pub fn run_program(path: String) -> Result<(), String> {
    if path.contains("powershell")
        || path.contains("node")
        || path.contains("bun")
        || (path.contains("cmd.exe") && path.contains("System"))
    {
        println!("{}", file_name_without_extension(&path));
        Command::new("cmd")
            .current_dir("C:/Users/")
            .args(&[
                "/C",
                &format!("start cmd /k {}", file_name_without_extension(&path)),
            ])
            .spawn()
            .map_err(|err| err.to_string())?;
    } else {
        println!("{}", path);
        Command::new(path).spawn().map_err(|err| err.to_string())?;
    }
    Ok(())
}
