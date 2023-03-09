use druid::{Lens, Data};
use im::Vector;

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    pub characters: Vector<Character>,
}

#[derive(Clone, Data, Lens, Default)]
pub struct Character {
    name: String,
    age: u32,
    health: u32,
}