use std::env;
pub fn add_drive_to_path(s: &str) -> String {
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
