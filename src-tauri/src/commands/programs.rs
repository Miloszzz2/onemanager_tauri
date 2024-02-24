use std::process::Command;
#[tauri::command]
pub(crate) fn getprogrampaths() -> Result<Vec<String>, String> {
    let apps = installed::list().map_err(|err| err.to_string())?;
    let mut options = vec![];

    for app in apps {
        let x = app.dump();
        if
            let Some(display_icon) = x
                .lines()
                .find(|line| {
                    line.starts_with("DisplayIcon") &&
                        line.contains(".exe") &&
                        !line.contains("uninstall") &&
                        !line.contains("Uninstall") &&
                        !line.contains("unins") &&
                        !line.contains("Installer") &&
                        !line.contains("setup") &&
                        !line.contains("{")
                })
        {
            let display_icon_without_prefix = sanitize_path(
                &display_icon.trim_start_matches("DisplayIcon:").trim().replace("\"", "")
            );

            options.push(display_icon_without_prefix.to_string());
        }
    }
    Ok(options)
}

fn sanitize_path(path: &str) -> String {
    if let Some(index) = path.find(',') { path[..index].to_string() } else { path.to_string() }
}
#[tauri::command]
pub(crate) fn runprogram(path: String) -> Result<(), String> {
    println!("{path}");
    Command::new(path)
        .spawn()
        .map_err(|err| err.to_string())?;
    Ok(())
}
