use byteorder::{BigEndian, ReadBytesExt};
use crate::PacketCodec;
use std::io::{Error, ErrorKind};
use std::marker::Sized;
use std::mem::transmute;
use std::convert::TryInto;

macro_rules! int_base {
    ($Type:ident, $size:expr) => {
        impl PacketCodec for $Type {
            fn to_mc_bytes(&self) -> Vec<u8> {
                unsafe {
                    let bytes: [u8; $size] = transmute(self.clone().to_be());
                    bytes.to_vec()
                }
            }

            fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
                if (bytes.len() < $size) {
                    return Err(Error::new(ErrorKind::InvalidData, "Byte buffer too short to parse data"));
                }

                let mut num_bytes = [0u8; $size];
                let mut index = 0;
                for byte in bytes.drain(0..$size) {
                    num_bytes[index] = byte;
                    index += 1;
                }

                Ok($Type::from_be_bytes(num_bytes))
            }
        }
    }
}

macro_rules! int_codec {
    ($Type:ident) => {
        int_base!($Type, std::mem::size_of::<$Type>());
    }
}

int_codec!(u8);
int_codec!(u8);