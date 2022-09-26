use crate::Result;
use crate::fs;
use crate::Jam;
use std::path::Path;

pub struct Jar {
    pub(crate) cat: u8,
    pub(crate) ty: u8,
    pub(crate) data: Vec<u8>,
}

impl Jar {
    pub fn new(cat: u8, ty: u8, data: Vec<u8>) -> Self {
        Self {
            cat,
            ty,
            data,
        }
    }

    pub(crate) fn pack(mut self) -> Vec<u8> {
        let mut out = vec![self.cat, self.ty];
        out.append(&mut self.data);
        out
    }

    pub fn unpack(self) -> Vec<u8> {
        self.data
    }
}

pub async fn jar(asset: impl Into<String>) -> Result<Jar> {
    let asset = asset.into();
    let path = Path::new(&asset)
        .with_extension("jam");

    match fs::read_to_bytes(path.to_str().unwrap()).await {
        Ok(b) => Ok(Jar::new(
            b[0],
            b[1],
            b[2..b.len()].to_vec(),
        )),
        Err(e) => Err(e),
    }
}

pub async fn open<T: 'static + Jam>(asset: impl Into<String>) -> Result<T> {
    let jar = jar(asset.into())
        .await?;
    T::decode(jar.ty, jar.data)
}