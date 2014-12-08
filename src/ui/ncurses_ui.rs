extern crate ncurses;
use self::ncurses::*;

use stats::{Stats};
use map::{Map};
use tile::{Tile};
use game_entity::{Entity};
use session::{Session};
use ui::static_ui;
use builders;
use static_data;

pub fn run() {
    init_curses_for_wasteland();

    let s: Stats = Stats::new();
    let mut m: Map = Map::new(30u32, 30u32);
    let session: Box<Session> = builders::session_builder::build_session();

    m.name("The badlands".to_string());
    m.randomize();
}

/// Configure ncurses to use colors for the game (init colors go here)
fn init_curses_for_wasteland() {
    initscr();
    keypad(stdscr, true);
    noecho();

    init_wasteland_colors();
}

/// Initialize the color pairs for the game
fn init_wasteland_colors() {
    start_color();
    init_color(static_ui::c_back, 0,0,0);
    init_color(static_ui::c_front, 120, 120, 120);
    init_color(static_ui::c_map_grass, 0, 255, 0);
    init_color(static_ui::c_map_hero, 255, 0, 0);
    init_color(static_ui::c_map_border, 165, 42, 0);

    init_pair(static_ui::c_grass_pair,
              static_ui::c_map_grass,
              static_ui::c_back);

    init_pair(static_ui::c_hero_pair,
              static_ui::c_map_hero,
              static_ui::c_back);

    init_pair(static_ui::c_border_pair,
              static_ui::c_map_border,
              static_ui::c_back);
}
