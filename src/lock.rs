use crate::fs;
use crate::util;
use std::collections::HashMap;
use sha1::{Sha1, Digest};

/// Utility for handling JamPack.lock files
pub(crate) struct Lock {
    files: HashMap<String, String>,
}

impl Lock {
    pub async fn load() -> Self {
        match fs::exists("./JamPack.lock").await {
            false => Self::create_with_defaults().await,
            true => Self::load_from_file(
                String::from_utf8(fs::read_to_bytes("./JamPack.lock").await.unwrap()).unwrap()
            ).await,
        }
    }

    pub async fn save(self) {
        let mut file = DEFAULT_LOCK.to_owned();

        for (path, hash) in self.files {
            file += "\n";
            file += path.as_str();
            file += "\n";
            file += hash.as_str();
        }

        fs::save("./JamPack.lock", file.as_bytes().to_vec()).await
    }

    async fn load_from_file(file: impl Into<String>) -> Self {
        let file = file.into();
        let lines: Vec<&str> = file.split("\n").collect();

        let mut files: HashMap<String, String> = HashMap::new();
        let mut cur_path: Option<String> = None;
        for line in lines {
            if line.starts_with("#") { continue; }
            match cur_path {
                // Add file to list
                Some(p) => {
                    let _ = files.insert(p, line.to_owned());
                    cur_path = None;
                },
                // Set current path to line
                None => cur_path = Some(line.to_owned()),
            }
        }

        Self {
            files,
        }
    }

    async fn create_with_defaults() -> Self {
        fs::save(
            "./JamPack.lock",
            DEFAULT_LOCK.as_bytes().to_vec(),
        ).await;

        Self {
            files: HashMap::new(),
        }
    }

    pub fn set_file(&mut self, path: impl Into<String>, hash: String) {
        let path = path.into();
        let _ = self.files.insert(path, hash);
    }
    pub fn remove_file(&mut self, path: impl Into<String>) {
        let path = path.into();
        let _ = self.files.remove(&path);
    }
    pub fn get_file(&self, path: impl Into<String>) -> Option<String> {
        let path = path.into();
        match self.files.get(&path) {
            Some(f) => Some(f.clone()),
            None => None,
        }
    }
    pub fn files(&self) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        for file in self.files.keys() {
            files.push(file.clone());
        }
        files
    }

    pub fn gen_hash(data: &Vec<u8>) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        
        let mut hash = String::new();
        for n in hasher.finalize().iter() {
            hash += util::u8_to_string(*n).as_str();
        }
        hash
    }
}

const DEFAULT_LOCK: &str = include_str!("./Default.lock");