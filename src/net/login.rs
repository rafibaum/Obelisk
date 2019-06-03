use super::codec;
use super::{Packet, PlayerSocket};
use serde_json::json;
use std::io::{Error, ErrorKind};
use uuid::Uuid;

pub fn handle_login(socket: &mut PlayerSocket, packet: &mut Packet) -> Result<Option<Uuid>, Error> {
    if packet.id == 0 {
        // Take in username and generate uuid, then use a central command to create a player
        // and return some sort of reference
        let username = read_login_start(socket, packet)?;
        println!("Received connection from {}", username);
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, username.as_bytes());

        let player = socket
            .server
            .write()
            .unwrap()
            .create_player(uuid.clone(), username.clone());
        send_login_success(socket, &uuid, &username);

        Ok(Some(uuid))
    } else {
        Err(Error::new(
            ErrorKind::InvalidData,
            "Unsupported login packet id",
        ))
    }
}

fn read_login_start(socket: &mut PlayerSocket, packet: &mut Packet) -> Result<String, Error> {
    codec::read_string(&mut packet.data)
}

fn send_login_success(socket: &mut PlayerSocket, uuid: &Uuid, username: &str) {
    let mut data = Vec::new();
    data.append(&mut codec::encode_string(&uuid.to_hyphenated().to_string()));
    data.append(&mut codec::encode_string(username));

    socket.send_packet(0x2, data)
}

fn send_login_disconnect(socket: &mut PlayerSocket) {
    let mut data = Vec::new();

    let chat = json!({
        "text": "Login failed"
    })
    .to_string();

    data.append(&mut codec::encode_string(&chat));

    socket.send_packet(0x0, data)
}
