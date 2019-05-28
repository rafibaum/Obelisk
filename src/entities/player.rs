use uuid::Uuid;

pub struct Player {
    pub uuid: Uuid,
    pub username: String,
    pub entity_id: i32
}

pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator
}