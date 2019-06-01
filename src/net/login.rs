/*use std::net::TcpStream;
use std::io;
use uuid::Uuid;
use super::codec;
use crate::entities::player;

pub fn handle_login(stream: &mut TcpStream) -> Result<player::Player, io::Error> {
    let header = super::read_header(stream)?;
    if header.id == 0 {
        let username = read_login_start(stream)?;
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, username.as_bytes());
        let eid = rand::random();

        send_login_success(stream, &username, &uuid.to_hyphenated().to_string())?;

        return Ok(player::Player {
            uuid,
            username,
            entity_id: eid
        });
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported login packet"));
    }
}

fn read_login_start(stream: &mut TcpStream) -> Result<String, io::Error> {
    codec::read_string(stream)
}

fn send_login_success(stream: &mut TcpStream, username: &str, uuid: &str) -> Result<(), io::Error> {
    let mut data = Vec::new();
    data.append(&mut codec::encode_string(uuid));
    data.append(&mut codec::encode_string(username));

    super::send_packet(stream, 0x2, &data)
}*/
