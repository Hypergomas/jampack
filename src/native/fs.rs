use crate::Result;
use std::fs;

pub async fn read_to_bytes(path: impl Into<String>) -> Result<Vec<u8>> {
    match fs::read(path.into()) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}
