use crate::world::chunks::Column;
use std::collections::HashMap;
use crate::world::chunks::ChunkColumn;
use crate::entities::player;
use std::sync::Weak;

pub mod chunks;
pub mod palette;

pub struct World {
    pub gamemode: player::Gamemode,
    pub hardcore: bool,
    pub dimension: Dimension,
    pub difficulty: Difficulty,
    pub level_type: LevelType,
    pub columns: HashMap<Column, ChunkColumn>
}

impl World {
    pub fn get_column(&self, column: &Column) {
        
    }
}

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub world: Weak<World>,
}

impl Location {
    pub fn to_vector(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Into<Vector> for Location {
    fn into(self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Dimension {
    Overworld = 0,
    End = 1,
    Nether = -1,
}

#[derive(Copy, Clone)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Copy, Clone)]
pub enum LevelType {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Default_1_1,
}

impl LevelType {
    pub fn to_string(&self) -> &str {
        match self {
            LevelType::Default => "default",
            LevelType::Flat => "flat",
            LevelType::LargeBiomes => "largeBiomes",
            LevelType::Amplified => "amplified",
            LevelType::Default_1_1 => "default_1_1",
        }
    }
}
