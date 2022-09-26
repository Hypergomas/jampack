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