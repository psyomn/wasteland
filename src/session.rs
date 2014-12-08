use game_entity::{Entity};

pub struct Session {
    player: Entity,
}

impl Session {
    pub fn new() -> Session { Session { player: Entity::new() } }

    pub fn set_player_name(&mut self, name: String) {
        self.player.name(name);
    }
}
