use std::net::TcpStream;
use std::io;
use crate::entities::player;
use crate::world;
use super::codec;

pub fn handle_play(stream: &mut TcpStream, server: &crate::Obelisk, player: &player::Player) -> Result<(), io::Error> {
    send_join_game(stream, server, player)?;

    Ok(())
}

fn send_join_game(stream: &mut TcpStream, server: &crate::Obelisk, player: &player::Player) -> Result<(), io::Error> {
    let mut data = Vec::new();
    data.append(&mut codec::encode_int(player.entity_id));

    let spawn_world = server.spawn_location.world.upgrade().expect("Spawn world does not exist");
    let mut gamemode = spawn_world.gamemode as u8;

    if spawn_world.hardcore {
        gamemode |= 0b100;
    }

    data.append(&mut codec::encode_ubyte(gamemode));
    data.append(&mut codec::encode_int(spawn_world.dimension as i32));
    data.append(&mut codec::encode_ubyte(spawn_world.difficulty as u8));
    data.append(&mut codec::encode_ubyte(0)); // Ignored max players
    data.append(&mut codec::encode_string(spawn_world.level_type.to_string()));
    data.append(&mut codec::encode_bool(false)); // Optional debug values

    super::send_packet(stream, 0x25, &data)
}

fn send_spawn_position(stream: &mut TcpStream, server: &crate::Obelisk) -> Result<(), io::Error> {
    let data = codec::encode_position(&server.spawn_location.to_vector());

    super::send_packet(stream, 0x49, &data)
}

fn send_player_abilities(stream: &mut TcpStream, server: &crate::Obelisk) -> Result<(), io::Error> {
    let mut data = Vec::new();

    let abilities = if server.spawn_location.world.upgrade()
        .expect("Spawn world does not exist").gamemode == world::Gamemode::Survival {
        0
    } else {
        0b1111
    };

    data.append(&mut codec::encode_byte(abilities));
    data.append(&mut codec::encode_float(0.05)); // Flying speed
    data.append(&mut codec::encode_float(0.1)); // FOV modifier

    super::send_packet(stream, 0x2E, &data)
}