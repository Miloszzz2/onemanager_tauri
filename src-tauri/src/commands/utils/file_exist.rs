use std::path::Path;
pub fn file_exists(file_path: String) -> bool {
    let path = Path::new(&file_path);
    path.exists() && path.is_file()
}
