#![feature(globs)]

extern crate ncurses;
extern crate wasteland;

use std::char;
use ncurses::*;

use wasteland::stats::{Stats};
use wasteland::map::{Map};
use wasteland::tile::{Tile};
use wasteland::game_entity::{Entity};

// mod stats;
// mod map;
// mod tile;
// mod game_entity;

fn main() {
    let s: Stats = Stats::new();
    let mut m: Map = Map::new(30u32, 30u32);

    m.name("The badlands".to_string());

    println!("{}", m);
}
