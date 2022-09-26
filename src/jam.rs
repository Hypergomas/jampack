use crate::Result;

pub trait Jam: Sized {
    fn decode(ty: u8, data: Vec<u8>) -> Result<Self>;
}