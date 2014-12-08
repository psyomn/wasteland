#![feature(globs)]

extern crate ncurses;
extern crate wasteland;

use std::char;
use ncurses::*;

use wasteland::stats::{Stats};
use wasteland::map::{Map};
use wasteland::tile::{Tile};
use wasteland::game_entity::{Entity};
use wasteland::session::{Session};
use wasteland::builders;
use wasteland::static_data;

fn main() {
    let s: Stats = Stats::new();
    let mut m: Map = Map::new(30u32, 30u32);
    let session: Box<Session> = builders::session_builder::build_session();

    m.name("The badlands".to_string());
    m.randomize();

    println!("{}", m);
    println!("{}", static_data::begin_text);
}
