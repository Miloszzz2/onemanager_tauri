pub fn sanitize_path(path: &str) -> String {
    if let Some(index) = path.find(',') {
        path[..index].to_string()
    } else {
        path.to_string()
    }
}
