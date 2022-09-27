use crate::Result;
use std::{fs, io::Write, path::Path};

pub async fn read_to_bytes(path: impl Into<String>) -> Result<Vec<u8>> {
    match fs::read(path.into()) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn save(path: impl Into<String>, data: Vec<u8>) {
    let path: String = path.into();

    // Create directory if it doesn't exist
    let dir = {
        let path_ref = Path::new(&path);
        let fname = path_ref.file_name().unwrap();
        let fname = fname.to_str().unwrap();
        path.strip_suffix(fname).unwrap()
    };
    let _ = fs::create_dir_all(dir);

    let mut file = fs::File::create(path)
        .expect("Could not create file");
    
    file.write(&data)
        .expect("Could not write data to file");
}

pub async fn delete(path: impl Into<String>) {
    let path: String = path.into();
    let path = Path::new(&path);

    if path.is_file() {
        let _ = fs::remove_file(path);
    } else if path.is_dir() {
        let _ = fs::remove_dir_all(path);
    }
}

pub async fn exists(path: impl Into<String>) -> bool {
    let path = path.into();
    Path::new(&path).exists()
}