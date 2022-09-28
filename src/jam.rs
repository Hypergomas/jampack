use crate::Result;

pub trait Jam: Sized {
    fn unjar(ty: u8, data: Vec<u8>) -> Result<Self>;
    fn jam_idx() -> u8;
}