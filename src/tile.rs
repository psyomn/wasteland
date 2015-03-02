use std::fmt;
use game_entity::{Entity};

pub struct Tile {
    entities: Vec<Box<Entity>>,
    player: Option<Box<Entity>>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            entities: vec![],
            player: None,
        }
    }

    /// Count the number of entities on the current tile
    pub fn count(&self) -> uint {
        self.entities.len()
    }

    pub fn add_entity(&mut self, e: Box<Entity>) {
        self.entities.push(e);
    }

    pub fn set_player(&mut self, player: Option<Box<Entity>>) {
        self.player = player;
    }

}

impl Clone for Tile {
    fn clone(&self) -> Tile { Tile{ entities: vec![], player: None} }
}

impl fmt::Show for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = self.entities.len();
        let mut res : fmt::Result;
        if c > 0 {
            res = write!(f, "{:<2}", c);
        }
        else {
            res = write!(f, "{}", ". ");
        }
        return res;
    }
}
