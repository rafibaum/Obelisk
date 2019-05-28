use std::net::TcpStream;
use std::io;
use serde_json::json;
use uuid::Uuid;
use super::codec;

pub fn handle_login(stream: &mut TcpStream) -> Result<(), io::Error> {
    let header = super::read_header(stream)?;
    if header.id == 0 {
        let username = read_login_start(stream)?;
        send_login_success(stream, &username)?;
    }

    Ok(())
}

fn read_login_start(stream: &mut TcpStream) -> Result<String, io::Error> {
    codec::read_string(stream)
}

fn send_login_success(stream: &mut TcpStream, username: &str) -> Result<(), io::Error> {
    let uuid = Uuid::new_v5(Uuid::borrow(), username.as_bytes());
    let mut data = Vec::new();
    data.append(codec::encode_string(&uuid.to_hyphenated().to_string()));
    data.append(codec::encode_string(username));

    super::send_packet(stream, 2, &data)
}