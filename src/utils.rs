use std::error::Error;

use serde::Serialize;

pub fn strip_trailing_newline(s: &str) -> String {
    s.strip_suffix("\r\n")
        .or(s.strip_suffix("\n"))
        .unwrap_or(s)
        .to_string()
}

pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let bytes_vec: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect();

    (bytes_vec[0], bytes_vec[1], bytes_vec[2])
}

pub fn check_dirty<T, F>(current_val: &bool, items: &[T], check_fn: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    *current_val || items.iter().any(check_fn)
}
