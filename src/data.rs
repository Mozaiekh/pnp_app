use druid::{Lens, Data};
use im::Vector;
use serde::{Serialize, Deserialize};

#[derive(Clone, Data, Lens, Default, PartialEq, Eq)]
pub struct AppState {
    pub characters: Vector<Character>,
    pub chargen_name: String,
    pub chargen_age: String,
    pub chargen_sex: String,
    pub chargen_race: String,
    pub chargen_occupation: String,
}

#[derive(Clone, Data, Lens, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub age: u32,
    pub sex: String,
    pub race: String,
    pub occupation: String,
    pub action: u32,
    pub knowledge: u32,
    pub social: u32,
    pub health: u32,
}