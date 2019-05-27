use std::net::TcpStream;
use std::io::Read;

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

pub fn read_long(stream: &mut TcpStream) -> i64 {
    let mut buffer = [0; 8];
    stream.read(&mut buffer).unwrap();
    i64::from_be_bytes(buffer)
}

pub fn read_ushort(stream: &mut TcpStream) -> u16 {
    let mut buffer = [0; 2];
    stream.read(&mut buffer).unwrap();
    u16::from_be_bytes(buffer)
}

pub fn read_varint(stream: &mut TcpStream) -> i32 {
    let mut bytes_read = 0;
    let mut result: i32 = 0;
    loop {
        let mut buffer = [0];
        stream.read(&mut buffer).unwrap();

        let value = (buffer[0] & 0b01111111) as i32;
        result |= value << (7 * bytes_read);
        bytes_read += 1;

        if bytes_read > 5 {
            panic!("VarInt is too big");
        }

        if buffer[0] & 0b10000000 == 0 {
            break;
        }
    }

    result
}

pub fn read_string(stream: &mut TcpStream) -> String {
    let length = read_varint(stream) as usize;
    let mut buffer = vec![0; length];

    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer).to_string()
}