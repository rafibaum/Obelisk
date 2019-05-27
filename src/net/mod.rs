use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};

pub mod codec;
mod status;

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
        status::read_status(stream);
    } else if state == 2 {
        // Login
    } else {
        // Error
    }
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