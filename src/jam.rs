use crate::Result;
use crate::Jar;

pub trait Jam: Sized {
    fn encode(&self) -> Jar;
    fn decode(ty: u8, data: Vec<u8>) -> Result<Self>;
    fn jam_type_idx() -> u8;
}