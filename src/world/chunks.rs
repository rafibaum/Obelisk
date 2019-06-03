use super::Location;

pub struct Block {
    pub location: Location,
    pub id: u32,
}

pub struct ChunkSection {
    pub blocks: [[[Block; 16]; 16]; 16],
}

pub struct ChunkColumn {
    pub sections: [Option<ChunkSection>; 16],
    pub x: i32,
    pub z: i32,
}
