use obelisk::Obelisk;

pub mod entities;
pub mod net;

fn main() {
    let mut obelisk = Obelisk {
        players: Vec::new(),
        max_players: 10
    };

    obelisk.start();
}

pub mod obelisk {
    use super::entities::player;
    use super::entities::player::Player;
    use super::net;
    use uuid::Uuid;

    pub const VERSION: &str = "1.13.2";
    pub const PROTOCOL: u16 = 404;

    pub struct Obelisk {
        pub players: Vec<player::Player>,
        pub max_players: u32,
    }

    impl Obelisk {
        pub fn start(&mut self) {
            net::start(&self);
        }
    }
}