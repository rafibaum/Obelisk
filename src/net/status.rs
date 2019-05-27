use std::net::TcpStream;
use std::io::Read;
use std::io;
use serde_json::json;
use super::codec;

pub fn read_status(stream: &mut TcpStream) -> Result<(), io::Error> {
    loop {
        let header = super::read_header(stream)?;
        if header.id == 0 {
            send_status(stream)?
        } else if header.id == 1 {
            send_ping(stream)?
        }
    }
}

fn send_ping(stream: &mut TcpStream) -> Result<(), io::Error> {
    let mut payload = [0; 8];
    stream.read(&mut payload).unwrap();
    super::send_packet(stream, 1, &payload)
}

fn send_status(stream: &mut TcpStream) -> Result<(), io::Error> {
    let response = codec::encode_string(&json!({
    "version": {
        "name": "1.13.2",
        "protocol": 404
    },
    "players": {
        "max": 10,
        "online": 0,
        "sample": []
    },
    "description": {
        "text": "Hello world"
    }
    }).to_string());

    super::send_packet(stream, 0, &response)
}