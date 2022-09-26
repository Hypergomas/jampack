use crate::Result;
use crate::{Jam, Jar};

impl Jam for Vec<u8> {
    fn encode(&self) -> Jar {
        Jar::new(0, 0, self.clone())
    }

    fn decode(_: u8, data: Vec<u8>) -> Result<Self> {
        Ok(data.to_vec())
    }

    fn jam_type_idx() -> u8 {
        0
    }
}