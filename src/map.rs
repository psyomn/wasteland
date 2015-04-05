use std::fmt;

use tile::{Tile};
use game_entity::{Entity};
use helpers::map_helper;

pub struct Map {
    name: String,
    tiles: Vec<Vec<Tile>>,
    height: u32,
    width: u32,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name);

        for row in self.tiles.iter() {
            for tile in row.iter() {
                write!(f, "{:?}", tile);
            }
            writeln!(f, "");
        }

        write!(f, "")
    }
}

impl Map {
    pub fn new(w: u32, h: u32) -> Map {
        let mut map: Map =
            Map { name: "default map name".to_string(),
                  tiles:  vec![],
                  height: h,
                  width:  w };

        for curr_height in 0u32 .. h {
            /* make height of map */
            map.tiles.push(vec![]);
            for _ in 0u32 .. w {
                /* For each vector we place the tile */
                let chz = curr_height as usize;
                map.tiles[chz].push(Tile::new());
            }
        }

        return map;
    }

    /// Name/label of map
    pub fn name(&mut self, n: String) {
        self.name = n;
    }

    pub fn height(&self) -> u32 { self.height }

    pub fn width(&self)  -> u32 { self.width }

    /// Place an entity at a particular coordinate on map
    pub fn place_entity_at(&mut self, x: u32, y: u32, e: Box<Entity>) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.tiles[y][x].add_entity(e);
    }

    /// Randomize entities on map
    pub fn randomize(&mut self) {
        map_helper::poppulate_with_entities(self, 10);
    }

    /// Counts the number of entities on a particular tile
    pub fn count_at(&self, coord: (u32, u32)) -> u32 {
        let (x, y) = coord;
        self.tiles[y][x].count()
    }

    /// Give a reference to some tile, the particular player.
    pub fn place_player(&mut self, player: Box<Entity>) {
        self.tiles[0][0].set_player(Some(player));
    }

}
