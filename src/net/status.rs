use std::net::TcpStream;
use std::io::Read;
use std::io;
use serde::Serialize;
use serde_json::json;
use super::codec;
use crate::obelisk::Obelisk;
use core::borrow::Borrow;
use uuid::Uuid;

pub fn read_status(server: &Obelisk, stream: &mut TcpStream) -> Result<(), io::Error> {
    loop {
        let header = super::read_header(stream)?;
        println!("Received id: {}", header.id);
        if header.id == 0 {
            send_status(server, stream)?
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

fn send_status<'a>(server: &Obelisk, stream: &mut TcpStream) -> Result<(), io::Error> {
    #[derive(Serialize)]
    struct SamplePlayer<'a> {
        name: &'a str,
        id: String
    }

    let player_sample: Vec<SamplePlayer> = server.players.iter().take(5).map(|player| SamplePlayer {
        name: &player.username,
        id: player.uuid.to_hyphenated().to_string()
    }).collect();

    let response = codec::encode_string(&json!({
    "version": {
        "name": "1.13.2",
        "protocol": 404
    },
    "players": {
        "max": server.max_players,
        "online": server.players.len(),
        "sample": &player_sample,
    },
    "description": {
        "text": "Hello world"
    }
    }).to_string());
    println!("Send status");
    super::send_packet(stream, 0, &response)
}