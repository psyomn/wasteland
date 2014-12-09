use game_entity::{Entity};
use map::{Map};

pub struct Session {
    player: Entity,
    current_map: Box<Map>,
}

impl Session {
    pub fn new(m: Box<Map>) -> Session {
        Session { player: Entity::new(), current_map: m }
    }

    pub fn set_player_name(&mut self, name: String) {
        self.player.name(name);
    }

    pub fn set_current_map(&mut self, m: Box<Map>) {
        self.current_map = m;
    }
}
