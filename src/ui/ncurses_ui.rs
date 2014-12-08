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
    let s: Stats = Stats::new();
    let mut m: Map = Map::new(30u32, 30u32);
    let session: Box<Session> = builders::session_builder::build_session("hiro".to_string());

    m.name("The badlands".to_string());
    m.randomize();

    init_curses_for_wasteland();
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
    init_color(static_ui::C_BACK,         0,   0,   0);
    init_color(static_ui::C_FRONT,      120, 120, 120);
    init_color(static_ui::C_MAP_GRASS,    0, 255,   0);
    init_color(static_ui::C_MAP_HERO,   255,   0,   0);
    init_color(static_ui::C_MAP_BORDER, 165,  42,   0);

    init_pair(static_ui::C_GRASS_PAIR,
              static_ui::C_MAP_GRASS,
              static_ui::C_BACK);

    init_pair(static_ui::C_HERO_PAIR,
              static_ui::C_MAP_HERO,
              static_ui::C_BACK);

    init_pair(static_ui::C_BORDER_PAIR,
              static_ui::C_MAP_BORDER,
              static_ui::C_BACK);

    init_pair(static_ui::C_DEFAULT_PAIR,
              static_ui::C_BACK,
              static_ui::C_FRONT);

    bkgd(' ' as u32 | COLOR_PAIR(static_ui::C_DEFAULT_PAIR) as u32);
}

/// Anything we need to do to clean up ncurses invokation
fn ncurses_cleanup() {

}
