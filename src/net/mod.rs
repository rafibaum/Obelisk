use std::net::{TcpListener, TcpStream};
use std::io::{Write, ErrorKind};
use std::io;
use std::error::Error;

pub mod codec;
mod status;
mod login;

pub fn start() {
    let listener = TcpListener::bind("localhost:25565").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(_) => ()
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    match read_handshake(&mut stream) {
        Ok(_) => (),
        Err(error) => println!("TCP stream error: {}", error.description())
    }
}

fn read_handshake(stream: &mut TcpStream) -> Result<(), io::Error> {
    let (_length, _id) = read_header(stream)?;
    let _version = codec::read_varint(stream)?;
    let _address = codec::read_string(stream)?;
    let _port = codec::read_ushort(stream)?;
    let state = codec::read_varint(stream)?;
    if state == 1 {
        status::read_status(stream)?
    } else if state == 2 {
        login::handle_login(stream)?
    } else {
        return Err(io::Error::new(ErrorKind::InvalidData, "Handshake had invalid state"));
    }

    Ok(())
}

fn send_packet(stream: &mut TcpStream, id: i32, data: &[u8]) -> Result<(), io::Error> {
    let mut packet = Vec::new();
    let id = codec::encode_varint(id);
    let length = codec::encode_varint((id.len() + data.len()) as i32);
    packet.extend_from_slice(&length);
    packet.extend_from_slice(&id);
    packet.extend_from_slice(data);
    stream.write(&packet)?;
    Ok(stream.flush()?)
}

fn read_header(stream: &mut TcpStream) -> Result<(i32, i32), io::Error> {
    Ok((codec::read_varint(stream)?, codec::read_varint(stream)?))
}