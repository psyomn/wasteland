use std::fmt;
use tile::{Tile};

pub struct Map {
    name: String,
    tiles: Vec< Vec<Tile> >,
    height: u32,
    width: u32,
}

impl fmt::Show for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name);

        for row in self.tiles.iter() {
            for tile in row.iter() {
                write!(f, "{:2}", tile.count());
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

        for curr_height in range(0u32, h) {
            /* make height of map */
            map.tiles.push(vec![]);
            for _ in range(0u32, w) {
                /* For each vector we place the tile */
                map.tiles[curr_height as uint].push(Tile::new());
            }
        }

        return map;
    }

    pub fn name(&mut self, n: String) {
        self.name = n;
    }
}