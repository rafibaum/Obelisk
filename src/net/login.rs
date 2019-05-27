use std::net::TcpStream;
use std::io;
use serde_json::json;
use super::codec;

pub fn handle_login(stream: &mut TcpStream) -> Result<(), io::Error> {
    let header = super::read_header(stream)?;
    if header.id == 0 {
        let username = codec::read_string(stream)?;
        super::send_packet(stream, 0, &codec::encode_string(
            &json!({ "text": format!("Received login as: {}", username)}).to_string()))?
    }

    Ok(())
}