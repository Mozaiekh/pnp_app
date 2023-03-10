use druid::{Lens, Data};
use im::Vector;
use serde::{Serialize, Deserialize};

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    pub characters: Vector<Character>,
    pub chargen_name: String,
    pub chargen_age: String,
}

#[derive(Clone, Data, Lens, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub age: u32,
    pub sex: char,
    pub race: String,
    pub action: u32,
    pub knowledge: u32,
    pub social: u32,
    pub health: u32,
}