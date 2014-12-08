extern crate ncurses;
use self::ncurses::*;

use stats::{Stats};
use map::{Map};
use tile::{Tile};
use game_entity::{Entity};
use session::{Session};
use builders;
use static_data;

pub fn run() {
    let s: Stats = Stats::new();
    let mut m: Map = Map::new(30u32, 30u32);
    let session: Box<Session> = builders::session_builder::build_session();

    m.name("The badlands".to_string());
    m.randomize();

    println!("Ncurses UI here");
}
