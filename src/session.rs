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

    pub fn map_height(&self) -> i32 { self.current_map.height() as i32 }

    pub fn map_width(&self) -> i32 { self.current_map.width() as i32 }

    /// Counts the entities on a specific coordinate (in tile). Returns -1 if
    /// something is wrong with the given coordinates (out of bounds)
    pub fn map_count_at(&self, coord: (usize, usize)) -> i32 {
        let (x, y) = coord;
        let x32 = x;
        let y32 = y;

        // if self.map_height()    < y32
        //     || self.map_width() < x32
        //     || (x32 < 0 || y32 < 0) {

        //     /* hm maybe not idiomatic rust... */
        //     return -1;
        // }
        self.current_map.count_at(coord) as i32
    }

}
