use crate::world::World;
use std::sync::Weak;
use crate::world::Vector;
use super::Location;

pub struct Block {
    pub vector: Vector,
    pub id: u32,
    pub block_light: u8,
    pub sky_light: Option<u8>
}

pub struct ChunkSection {
    pub blocks: [[[Block; 16]; 16]; 16],
}

impl ChunkSection {
    pub fn new(column: Column, y: usize) {
        
    }

    pub fn get_block(&self, vector: &Vector) -> &Block {
        &self.blocks[vector.y as usize][vector.z as usize][vector.x as usize]
    }

    pub fn set_block(&mut self, block: Block) {
        let (x, y, z) = (block.vector.x, block.vector.y, block.vector.z);
        self.blocks[y as usize][z as usize][x as usize] = block
    }
}

pub struct ChunkColumn {
    pub sections: [Option<ChunkSection>; 16],
    pub x: i32,
    pub z: i32,
}

#[derive(Eq, Hash, PartialEq)]
pub struct Column {
    pub x: i32,
    pub z: i32,
}

impl From<Vector> for Column {
    fn from(vector: Vector) -> Self {
        Column {
            x: (vector.x / 32.0) as i32,
            z: (vector.z / 32.0) as i32,
        }
    }
}
