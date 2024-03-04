use std::process::Command;
use std::path::Path;

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
            let pathtocheck = format!(
                "C:/Users/PC/Documents/Projects/Rust/tauri/onemanager/public/app_icons/{}.png",
                file_name_without_extension(&display_icon_without_prefix)
            );

            if file_exists(display_icon_without_prefix.clone()) {
                if !file_exists(pathtocheck) {
                    let args = [
                        format!("'{}'", display_icon_without_prefix),
                        format!("'{}'", file_name_without_extension(&display_icon_without_prefix)),
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

                    println!("Python script output: {:?}", output);
                }
                options.push(display_icon_without_prefix.to_string());
            }
        }
    }
    Ok(options)
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
