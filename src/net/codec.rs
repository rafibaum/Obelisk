use std::io::{ErrorKind, Read};
use std::io;
use std::net::TcpStream;
use std::mem::transmute;

pub fn encode_bool(val: bool) -> Vec<u8> {
    if val {
        vec![1]
    } else {
        vec![0]
    }
}

pub fn encode_ubyte(num: u8) -> Vec<u8> {
    vec![num]
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

pub fn encode_string(string: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    let bytes = string.as_bytes();
    encoded.extend_from_slice(&encode_varint(bytes.len() as i32));
    encoded.extend_from_slice(bytes);

    encoded
}

pub fn read_long(stream: &mut TcpStream) -> Result<i64, io::Error> {
    let mut buffer = [0; 8];
    stream.read(&mut buffer)?;
    Ok(i64::from_be_bytes(buffer))
}

pub fn read_ushort(stream: &mut TcpStream) -> Result<u16, io::Error> {
    let mut buffer = [0; 2];
    stream.read(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

pub fn read_varint(stream: &mut TcpStream) -> Result<i32, io::Error> {
    let mut bytes_read = 0;
    let mut result: i32 = 0;
    loop {
        let mut buffer = [0];
        stream.read(&mut buffer)?;

        let value = (buffer[0] & 0b01111111) as i32;
        result |= value << (7 * bytes_read);
        bytes_read += 1;

        if bytes_read > 5 {
            return Err(io::Error::new(ErrorKind::InvalidData, "VarInt was too long"));
        }

        if buffer[0] & 0b10000000 == 0 {
            break;
        }
    }

    Ok(result)
}

pub fn read_string(stream: &mut TcpStream) -> Result<String, io::Error> {
    let length = read_varint(stream)? as usize;
    let mut buffer = vec![0; length];

    stream.read(&mut buffer).unwrap();
    match String::from_utf8(buffer) {
        Ok(s) => Ok(s),
        Err(_) => Err(io::Error::new(ErrorKind::InvalidData, "String had invalid data"))
    }
}