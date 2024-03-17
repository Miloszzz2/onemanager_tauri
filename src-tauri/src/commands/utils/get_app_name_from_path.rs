pub fn get_app_name_from_path(app: &str) -> String {
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
