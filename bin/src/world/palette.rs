use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

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
    pub default: bool,
}

pub fn generate_palette(json_str: &str) {

}
