use crate::error::CommonError;
use std::io::Write;

pub fn vector_to_string(byte_slice: &[u8]) -> Result<String, CommonError> {
    let word = str::from_utf8(byte_slice)?;
    Ok(word.to_string())
}

pub fn string_to_vector(input: &str) -> Vec<u8> {
    let trimmed_input = input.trim();
    let bytes: Vec<u8> = trimmed_input.chars().map(|c| c as u8).collect();
    let mut byte_vector = Vec::new();
    byte_vector
        .write_all(&bytes)
        .expect("Unable to create bytes");
    byte_vector
}
