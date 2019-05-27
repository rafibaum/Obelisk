use std::net::TcpStream;
use std::io;
use super::codec;

pub fn handle_login(stream: &mut TcpStream) -> Result<(), io::Error> {
    let (length, id) = super::read_header(stream)?;
    if id == 0 {
        let username = codec::read_string(stream)?;
        super::send_packet(stream, 0, &codec::encode_string(&format!("{{ \"text\": \"Received login as: {}\" }}", username)))?
    }

    Ok(())
}