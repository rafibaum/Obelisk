use obelisk::Obelisk;

pub mod entities;
pub mod net;
pub mod world;

fn main() {
    let mut obelisk = Obelisk {
        players: Vec::new(),
        max_players: 10,
        worlds: Vec::new()
    };

    obelisk.start();
}

pub mod obelisk {
    use uuid::Uuid;
    use crate::entities::player;
    use crate::net;
    use crate::world;

    pub const VERSION: &str = "1.13.2";
    pub const PROTOCOL: u16 = 404;

    pub struct Obelisk {
        pub players: Vec<player::Player>,
        pub max_players: u32,
        pub worlds: Vec<world::World>
    }

    impl Obelisk {
        pub fn start(&mut self) {
            self.worlds.push(world::World {
                gamemode: player::Gamemode::Creative,
                hardcore: false,
                dimension: world::Dimension::Overworld,
                difficulty: world::Difficulty::Peaceful,
                level_type: world::LevelType::Default
            });

            net::start(&self);
        }
    }
}