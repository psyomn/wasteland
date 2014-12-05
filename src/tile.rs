use std::fmt;
use game_entity::{Entity};

pub struct Tile {
    entities: Vec<Entity>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile { entities: vec![], }
    }

    /// Count the number of entities on the current tile
    pub fn count(&self) -> uint {
        self.entities.len()
    }
}

impl Clone for Tile {
    fn clone(&self) -> Tile { Tile{ entities: vec![], } }
}

impl fmt::Show for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.entities.len())
    }
}
