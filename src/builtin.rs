use crate::Result;
use crate::Jam;

impl Jam for Vec<u8> {
    fn decode(_: u8, data: Vec<u8>) -> Result<Self> {
        Ok(data.to_vec())
    }
}