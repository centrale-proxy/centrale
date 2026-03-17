use crate::error::WriterError;

pub fn vector_to_string(byte_slice: &[u8]) -> Result<String, WriterError> {
    let word = str::from_utf8(byte_slice)?;
    Ok(word.to_string())
}
