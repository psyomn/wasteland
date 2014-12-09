/* This contains the main window of the application */
extern crate ncurses;
use self::ncurses::*;

use std::io::timer::{sleep};
use std::time::duration::Duration;

use stats::{Stats};
use map::{Map};
use session::{Session};
use ui::static_ui;
use builders;
use static_data;

pub fn run() {
    make_game_data();
    init_curses_for_wasteland();
}

fn make_game_data() {
    let s: Stats = Stats::new();
    let mut m: Map = Map::new(30u32, 30u32);
    let session: Box<Session> = builders::session_builder::build_session("hiro".to_string());

    m.name("The badlands".to_string());
    m.randomize();
}

/// Configure ncurses to use colors for the game (init colors go here)
fn init_curses_for_wasteland() {
    initscr();
    init_wasteland_colors();
    keypad(stdscr, true);
    // curs_set(false); currently errs
    noecho();

    clear();

    attron(A_BOLD());
    attron(COLOR_PAIR(static_ui::C_HERO_PAIR));
    mv(0,0);
    printw("Hello world");
    attroff(COLOR_PAIR(static_ui::C_HERO_PAIR));
    mv(1,4);
    printw("Hello world");
    mv(2,8);
    printw("Hello world");
    refresh();
    sleep(Duration::seconds(1));
    attron(A_BOLD());

    endwin();
}

/// Initialize the color pairs for the game
fn init_wasteland_colors() {
    start_color();
    init_color(static_ui::C_BACK,         0,   0,   0);
    init_color(static_ui::C_FRONT,      400, 420, 420);
    init_color(static_ui::C_MAP_GRASS,    0, 255,   0);
    init_color(static_ui::C_MAP_HERO,   555,   0,   0);
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
              static_ui::C_FRONT,
              static_ui::C_BACK);

    bkgd(' ' as u32 | COLOR_PAIR(static_ui::C_DEFAULT_PAIR) as u32);
}

/// Anything we need to do to clean up ncurses invokation
fn ncurses_cleanup() {

}

