pub mod convert;
pub mod error;
pub mod payload;
pub mod random;

pub fn truncate(s: &str, max_len: usize) -> String {
    match s.char_indices().nth(max_len) {
        Some((idx, _)) => s[..idx].to_string(),
        None => s.to_string(),
    }
}
