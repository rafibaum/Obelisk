use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};

mod codec;

pub fn start() {
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
    let version = codec::read_varint(stream);
    let address = codec::read_string(stream);
    let port = codec::read_ushort(stream);
    let state = codec::read_varint(stream);
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
    let response = codec::encode_string("{
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
    let id = codec::encode_varint(id);
    let length = codec::encode_varint((id.len() + data.len()) as i32);
    packet.extend_from_slice(&length);
    packet.extend_from_slice(&id);
    packet.extend_from_slice(data);
    stream.write(&packet).unwrap();
    stream.flush().unwrap();
}

fn read_header(stream: &mut TcpStream) -> (i32, i32) {
    (codec::read_varint(stream), codec::read_varint(stream))
}