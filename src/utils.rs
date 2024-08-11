pub fn strip_trailing_newline(s: &str) -> String {
    s.strip_suffix("\r\n")
        .or(s.strip_suffix("\n"))
        .unwrap_or(s)
        .to_string()
}
