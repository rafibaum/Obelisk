use crate::entities::player;

pub struct World {
    pub gamemode: player::Gamemode,
    pub hardcore: bool,
    pub dimension: Dimension,
    pub difficulty: Difficulty,
    pub level_type: LevelType
}

pub enum Dimension {
    Nether,
    Overworld,
    End
}

pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard
}

pub enum LevelType {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Default_1_1
}