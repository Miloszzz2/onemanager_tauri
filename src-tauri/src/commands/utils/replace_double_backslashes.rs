pub fn replace_double_backslashes(s: &str) -> String {
    s.replace("\\\\", "\\")
}
