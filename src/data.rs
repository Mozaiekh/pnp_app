use druid::{Lens, Data};
use im::Vector;

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    pub characters: Vector<Character>,
    pub new_character: String,
}

#[derive(Clone, Data, Lens, Default)]
pub struct Character {
    pub name: String,
    pub age: u32,
    pub health: u32,
}