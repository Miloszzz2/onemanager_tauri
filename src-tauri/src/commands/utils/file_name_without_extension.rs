use std::path::Path;
pub fn file_name_without_extension(path: &str) -> String {
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
