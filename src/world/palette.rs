use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{Value, Map};

#[derive(Deserialize)]
pub struct PaletteEntry {
    #[serde(default)]
    pub properties: Option<HashMap<String, Vec<String>>>,

    pub states: Vec<PaletteState>,
}

#[derive(Deserialize)]
pub struct PaletteState {
    #[serde(default)]
    pub properties: Option<HashMap<String, String>>,

    pub id: u32,

    #[serde(default)]
    pub default: bool
}