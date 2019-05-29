use std::mem::transmute;
use byteorder::{BigEndian, WriteBytesExt};
use crate::world;
use tokio::net::TcpStream;
use tokio::io::{Error, ErrorKind};
use tokio::io::AsyncRead;
use tokio::prelude::Future;

/*pub fn encode_bool(val: bool) -> Vec<u8> {
    if val {
        vec![1]
    } else {
        vec![0]
    }
}

pub fn encode_byte(num: i8) -> Vec<u8> {
    unsafe {
        let bytes: [u8; 1] = transmute(num.to_be());
        bytes.to_vec()
    }
}

pub fn encode_ubyte(num: u8) -> Vec<u8> {
    vec![num]
}

pub fn encode_double(num: f64) -> Vec<u8> {
    let mut val = [0u8; 8];
    val.as_mut().write_f64::<BigEndian>(num)
        .expect("Unable to encode double");

    val.to_vec()
}

pub fn encode_float(num: f32) -> Vec<u8> {
    let mut val = [0u8; 4];
    val.as_mut().write_f32::<BigEndian>(num)
        .expect("Unable to encode float");

    val.to_vec()
}

pub fn encode_long(num: i64) -> Vec<u8> {
    unsafe {
        let bytes: [u8; 8] = transmute(num.to_be());
        bytes.to_vec()
    }
}

pub fn encode_int(num: i32) -> Vec<u8> {
    unsafe {
        let bytes: [u8; 4] = transmute(num.to_be());
        bytes.to_vec()
    }
}

pub fn encode_varint(mut num: i32) -> Vec<u8> {
    let mut result = Vec::new();

    loop {
        let mut value = num & 0b01111111;
        num >>= 7;
        if num == 0 {
            result.push(value as u8);
            break;
        } else {
            value |= 0b10000000;
            result.push(value as u8);
        }
    }

    result
}

pub fn encode_position(vector: &world::Vector) -> Vec<u8> {
    let value: i64 = ((vector.x as i64 & 0x3FFFFFF) << 38) |
        ((vector.y as i64 & 0xFFF) << 26) |
        (vector.z as i64 & 0x3FFFFFF);

    encode_long(value)
}

pub fn encode_string(string: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    let bytes = string.as_bytes();
    encoded.extend_from_slice(&encode_varint(bytes.len() as i32));
    encoded.extend_from_slice(bytes);

    encoded
}

pub fn read_long(stream: &mut TcpStream) -> Result<i64, Error> {
    let mut buffer = [0; 8];
    stream.read(&mut buffer)?;
    Ok(i64::from_be_bytes(buffer))
}

pub fn read_ushort(stream: &mut TcpStream) -> Result<u16, Error> {
    let mut buffer = [0; 2];
    stream.read(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}
*/

pub fn read_varint(bytes: &mut [u8]) -> Result<i32, Error> {
    let mut result: i32 = 0;
    let mut bytes_read = 0;

    for byte in bytes {
        let value = (*byte & 0b01111111) as i32;
        result |= value << (7 * bytes_read);
        bytes_read += 1;

        if bytes_read > 5 {
            return Err(Error::new(ErrorKind::InvalidData, "VarInt was too long"));
        }

        if *byte & 0b10000000 == 0 {
            break;
        }
    }

    Ok(result)
}

/*
pub fn read_string(stream: &mut TcpStream) -> Result<String, Error> {
    let length = read_varint(stream)? as usize;
    let mut buffer = vec![0; length];

    stream.read(&mut buffer)?;
    match String::from_utf8(buffer) {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "String had invalid data"))
    }
}
*/