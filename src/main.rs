use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

fn main() {
    let listener = TcpListener::bind("localhost:25565").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    read_handshake(&mut stream);
}

fn read_handshake(stream: &mut TcpStream) {
    let (length, id) = read_header(stream);
    let version = read_varint(stream);
    let address = read_string(stream);
    let port = read_ushort(stream);
    let state = read_varint(stream);
    if state == 1 {
        read_status(stream);
    } else if state == 2 {
        // Login
    } else {
        // Error
    }
}

fn read_status(stream: &mut TcpStream) {
    loop {
        let (length, id) = read_header(stream);
        if id == 0 {
            send_status(stream);
        } else if id == 1 {
            send_ping(stream);
        }
    }
}

fn send_ping(stream: &mut TcpStream) {
    let mut payload = [0; 8];
    stream.read(&mut payload).unwrap();
    send_packet(stream, 1, &payload);
}

fn send_status(stream: &mut TcpStream) {
    let response = encode_string("{
    \"version\": {
        \"name\": \"1.13.2\",
        \"protocol\": 404
    },
    \"players\": {
        \"max\": 100,
        \"online\": 5,
        \"sample\": [
            {
                \"name\": \"Cobol72\",
                \"id\": \"f2b92aaf-40cc-4272-ad73-179f1b624658\"
            }
        ]
    },
    \"description\": {
        \"text\": \"Hello world\"
    }
}");

    send_packet(stream, 0, &response);
}

fn send_packet(stream: &mut TcpStream, id: i32, data: &[u8]) {
    let mut packet = Vec::new();
    let id = encode_varint(id);
    let length = encode_varint((id.len() + data.len()) as i32);
    packet.extend_from_slice(&length);
    packet.extend_from_slice(&id);
    packet.extend_from_slice(data);
    stream.write(&packet).unwrap();
    stream.flush().unwrap();
}

fn encode_varint(mut num: i32) -> Vec<u8> {
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

fn encode_string(string: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    let bytes = string.as_bytes();
    encoded.extend_from_slice(&encode_varint(bytes.len() as i32));
    encoded.extend_from_slice(bytes);

    encoded
}

fn read_header(stream: &mut TcpStream) -> (i32, i32) {
    (read_varint(stream), read_varint(stream))
}

fn read_long(stream: &mut TcpStream) -> i64 {
    let mut buffer = [0; 8];
    stream.read(&mut buffer).unwrap();
    i64::from_be_bytes(buffer)
}

fn read_ushort(stream: &mut TcpStream) -> u16 {
    let mut buffer = [0; 2];
    stream.read(&mut buffer).unwrap();
    u16::from_be_bytes(buffer)
}

fn read_varint(stream: &mut TcpStream) -> i32 {
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

fn read_string(stream: &mut TcpStream) -> String {
    let length = read_varint(stream) as usize;
    let mut buffer = vec![0; length];

    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer).to_string()
}