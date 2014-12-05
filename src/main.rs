#![feature(globs)]

extern crate ncurses;

use std::char;
use ncurses::*;

use stats::{Stats};
use map::{Map};

mod stats;
mod map;
mod tile;
mod game_entity;

fn main() {
    let s: Stats = Stats::new();
    let mut m: Map = Map::new(30u32, 30u32);

    m.name("The badlands".to_string());

    println!("{}", m);
}
