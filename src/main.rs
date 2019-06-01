use std::sync::{Arc, RwLock};
use obelisk::Obelisk;
use crate::entities::player;
use crate::world::Location;

pub mod entities;
pub mod net;
pub mod world;

fn main() {
    let world = Arc::new(world::World {
        gamemode: player::Gamemode::Creative,
        hardcore: false,
        dimension: world::Dimension::Overworld,
        difficulty: world::Difficulty::Peaceful,
        level_type: world::LevelType::Default
    });

    let spawn_location = Location {
        x: 0.0,
        y: 128.0,
        z: 0.0,
        world: Arc::downgrade(&world)
    };

    let mut worlds = Vec::new();
    worlds.push(world);

    let mut obelisk = Obelisk {
        players: Vec::new(),
        max_players: 10,
        worlds,
        spawn_location
    };

    let mut obelisk = Arc::new(RwLock::new(obelisk));

    net::start(obelisk.clone());
}

pub mod obelisk {
    use crate::entities::player;
    use crate::world;
    use std::sync::Arc;

    pub const VERSION: &str = "1.13.2";
    pub const PROTOCOL: u16 = 404;

    pub struct Obelisk {
        pub players: Vec<player::Player>,
        pub max_players: u32,
        pub worlds: Vec<Arc<world::World>>,
        pub spawn_location: world::Location
    }
}
