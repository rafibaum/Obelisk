use super::codec;
use super::{Packet, PlayerSocket};
use serde::Serialize;
use serde_json::json;

pub fn read_status(socket: &mut PlayerSocket, packet: &Packet) {
    if packet.id == 0 {
        send_status(socket);
    } else if packet.id == 1 {
        send_ping(socket, packet);
    } else {
        unimplemented!()
    }
}

fn send_ping(socket: &mut PlayerSocket, packet: &Packet) {
    let mut payload = packet.data.clone();
    socket.send_packet(0x1, payload);
}

fn send_status<'a>(socket: &mut PlayerSocket) {
    #[derive(Serialize)]
    struct SamplePlayer<'a> {
        name: &'a str,
        id: String,
    }

    let response: Vec<u8>;

    {
        let server = socket.server.read().unwrap();

        let player_sample: Vec<SamplePlayer> = server
            .players
            .iter()
            .take(5)
            .map(|player| SamplePlayer {
                name: &player.username,
                id: player.uuid.to_hyphenated().to_string(),
            })
            .collect();

        response = codec::encode_string(
            &json!({
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
            })
            .to_string(),
        );
    }

    socket.send_packet(0x0, response);
}
