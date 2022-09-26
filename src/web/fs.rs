use crate::Result;
use gloo_net::http::Request;

pub async fn read_to_bytes(path: impl Into<String>) -> Result<Vec<u8>> {
    let path = path.into();
    match Request::get(path.as_str())
        .send().await {
        Ok(r) => match r.binary().await {
            Ok(b) => Ok(b),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub async fn save(path: impl Into<String>, data: Vec<u8>) {}