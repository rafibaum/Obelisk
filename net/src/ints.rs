use byteorder::{BigEndian, ReadBytesExt};
use crate::PacketCodec;
use std::io::{Error, ErrorKind};
use std::marker::Sized;
use std::mem::transmute;
use std::convert::TryInto;

macro_rules! int_codec {
    ($Type:ident) => {
        const SIZE_$Type: usize = std::mem::size_of::<$Type>();
        impl PacketCodec for $Type {
            fn to_mc_bytes(&self) -> Vec<u8> {
                unsafe {
                    let bytes: [u8; SIZE_$Type] = transmute(self.clone().to_be());
                    bytes.to_vec()
                }
            }

            fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
                let bytes: [u8; SIZE_$Type] = bytes.drain(0..SIZE_$Type).try_into()?;
                Ok($Type::from_be_bytes(bytes))
            }
        }
    }
}

//int_codec!(u16);
const SIZE_u16: usize = std::mem::size_of::<u16>();
impl PacketCodec for u16 {

    fn to_mc_bytes(&self) -> Vec<u8> {
        unsafe {
            let bytes: [u8; SIZE_u16] = transmute(self.clone().to_be());
            bytes.to_vec()
        }
    }

    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        let bytes: [u8; SIZE_u16] = bytes.drain(0..SIZE_u16).try_into()?;
        Ok(u16::from_be_bytes(bytes))
    }
}