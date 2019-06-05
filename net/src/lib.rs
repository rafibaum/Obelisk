use std::io::Error;

mod ints;

pub trait PacketCodec where Self: Sized {
    fn to_mc_bytes(&self) -> Vec<u8>;
    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error>;
}