use crate::entities::player;
use crate::entities::player::Player;
use crate::world::palette::PaletteEntry;
use crate::world::Location;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub mod entities;
pub mod net;
pub mod world;

pub const VERSION: &str = "1.13.2";
pub const PROTOCOL: u16 = 404;

pub struct Obelisk {
    pub players: HashMap<Uuid, Player>,
    pub max_players: u32,
    pub worlds: Vec<Arc<world::World>>,
    pub spawn_location: world::Location,
}

fn main() {
    let world = Arc::new(world::World {
        gamemode: player::Gamemode::Creative,
        hardcore: false,
        dimension: world::Dimension::Overworld,
        difficulty: world::Difficulty::Peaceful,
        level_type: world::LevelType::Default,
        columns: HashMap::new(),
    });

    let spawn_location = Location {
        x: 0.0,
        y: 128.0,
        z: 0.0,
        world: Arc::downgrade(&world),
    };

    let mut worlds = Vec::new();
    worlds.push(world);

    let mut obelisk = Obelisk {
        players: HashMap::new(),
        max_players: 10,
        worlds,
        spawn_location,
    };

    let mut obelisk = Arc::new(RwLock::new(obelisk));

    net::start(obelisk.clone());

    let palette_json = fs::read_to_string("/home/rafi/blocks.json").expect("Could not read file");
    let palette: HashMap<String, PaletteEntry> =
        serde_json::from_str(&palette_json).expect("Failed to parse json");
}

impl Obelisk {
    pub fn create_player(&mut self, uuid: Uuid, username: String) -> &Player {
        self.players.insert(
            uuid.clone(),
            Player {
                uuid: uuid.clone(),
                username,
                entity_id: rand::random(),
                location: self.spawn_location.clone(),
            },
        );

        self.players.get(&uuid).unwrap()
    }
}
