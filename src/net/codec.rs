use crate::world;
use crate::world::chunks::{ChunkColumn, ChunkSection};
use byteorder::{BigEndian, WriteBytesExt};
use std::mem::transmute;
use tokio::io::{Error, ErrorKind};

pub fn encode_bool(val: bool) -> Vec<u8> {
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
    val.as_mut()
        .write_f64::<BigEndian>(num)
        .expect("Unable to encode double");

    val.to_vec()
}

pub fn encode_float(num: f32) -> Vec<u8> {
    let mut val = [0u8; 4];
    val.as_mut()
        .write_f32::<BigEndian>(num)
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
    let value: i64 = ((vector.x as i64 & 0x3FFFFFF) << 38)
        | ((vector.y as i64 & 0xFFF) << 26)
        | (vector.z as i64 & 0x3FFFFFF);

    encode_long(value)
}

pub fn encode_string(string: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    let bytes = string.as_bytes();
    encoded.extend_from_slice(&encode_varint(bytes.len() as i32));
    encoded.extend_from_slice(bytes);

    encoded
}

pub fn encode_chunk_column(column: &ChunkColumn) {
    let mut data = Vec::new();
    data.append(&mut encode_int(column.x));
    data.append(&mut encode_int(column.z));
    data.append(&mut encode_bool(true)); // Full chunk

    let mut mask: u8 = 0;
    for section in &column.sections {
        mask >>= 1;
        match section {
            Some(_) => mask |= 0b10000000,
            None => (),
        }
    }
    data.append(&mut encode_ubyte(mask));
}

pub fn encode_chunk_section(section: &ChunkSection) {
    let mut data = Vec::new();
    data.append(&mut encode_ubyte(14)); // Bits per block
    //Empty palette for direct usage

}

pub fn encode_ids(ids: Vec<u32>, size: i32) -> Vec<u64> {
    let mut data = Vec::new();
    let mut offset = 64 - size;
    let mut long: u64 = 0;
    for id in ids {
        let id = id as u64;
        if offset >= 0 {
            long |= id << offset as u64;
            offset -= size;
        } else {
            long |= id >> -offset as u64;
            data.push(long);
            offset += 64;
            long = id.overflowing_shl(offset as u32).0;
        }
    }

    if long != 0 {
        data.push(long);
    }

    data
}

pub fn read_long(bytes: &mut Vec<u8>) -> i64 {
    let mut num_bytes = [0u8; 8];
    let mut byte_slice = bytes.drain(..8).into_iter();
    for i in 0..8 {
        num_bytes[i] = byte_slice.next().unwrap();
    }

    i64::from_be_bytes(num_bytes)
}

pub fn read_ushort(bytes: &mut Vec<u8>) -> u16 {
    let mut num_bytes = [0u8; 2];
    let mut byte_slice = bytes.drain(..2).into_iter();
    for i in 0..2 {
        num_bytes[i] = byte_slice.next().unwrap();
    }

    u16::from_be_bytes(num_bytes)
}

pub fn read_varint(bytes: &mut Vec<u8>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    let mut bytes_read = 0;

    for byte in bytes.iter() {
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

    bytes.drain(..bytes_read);

    Ok(result)
}

pub fn read_string(bytes: &mut Vec<u8>) -> Result<String, Error> {
    let length = read_varint(bytes)? as usize;
    let drained_bytes = bytes.drain(..length);
    let mut char_vec = Vec::new();
    for b in drained_bytes {
        char_vec.push(b);
    }

    match String::from_utf8(char_vec) {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::new(
            ErrorKind::InvalidData,
            "String had invalid data",
        )),
    }
}
