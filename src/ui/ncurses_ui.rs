/* This contains the main window of the application */
extern crate ncurses;
use self::ncurses::*;

use std::old_io::timer::{sleep};
use std::time::duration::Duration;

use session::{Session};
use ui::static_ui;
use helpers::session_helper::{make_game_data};
use static_data;

/// Main game loop is here
pub fn run() {
    let session = make_game_data();
    init_curses_for_wasteland();

    loop {
        let inp = getch();

        if inp == 27 { break; }

        clear();
        draw_border(&*session);
        draw_map_contents(&*session);
        refresh();

        match inp {
            KEY_UP => {},
            KEY_DOWN => {},
            KEY_LEFT => {},
            KEY_RIGHT => {},

            /* Any other key should not do anything, and neither force refresh */
            _ => {continue;},
        }
    }

    ncurses_cleanup();
}


/// Draw the border that goes around the map. That means that when drawing map,
/// we need to specify an offset of +1 for x and y.
fn draw_border(s: &Session) {

    attron(A_BOLD());
    attron(COLOR_PAIR(static_ui::C_BORDER_PAIR));

    let offset = 2;
    let total_width = s.map_width() + offset;
    let total_height = s.map_height();

    /* Top and bottom borders */
    for y in vec![0, total_height + 1].iter() {
        for x in range(0, total_width) {
            mv(*y, x);
            printw(" ");
        }
    }

    /* Left and right borders */
    for x in vec![0, total_width - 1].iter() {
        for y in range(0, total_height) {
            mv(y, *x);
            printw(" ");
        }
    }

    attroff(COLOR_PAIR(static_ui::C_BORDER_PAIR));
    attroff(A_BOLD());

}

/// Draw the map contents within the borders
fn draw_map_contents(s: &Session) {
    let w = s.map_width();
    let h = s.map_height();

    for y in range(0, h) {
        for x in range(0, w) {
            let coord = (x as uint, y as uint);
            mv(y+1, x+1);
            if s.map_count_at(coord) > 0 {
                /* Entities exist on here */
                attron(COLOR_PAIR(static_ui::C_HERO_PAIR));
                printw("E");
                attroff(COLOR_PAIR(static_ui::C_HERO_PAIR));
            }
            else {
                /* Empty (grass for now) */
                attron(COLOR_PAIR(static_ui::C_GRASS_PAIR));
                printw(",");
                attroff(COLOR_PAIR(static_ui::C_GRASS_PAIR));
            }
        }
    }
}

/// Configure ncurses to use colors for the game (init colors go here)
fn init_curses_for_wasteland() {
    initscr();
    init_wasteland_colors();
    keypad(stdscr, true);
    // curs_set(false); currently errs
    noecho();
}

/// Initialize the color pairs for the game
fn init_wasteland_colors() {
    start_color();
    init_color(static_ui::C_BACK,         0,   0,   0);
    init_color(static_ui::C_FRONT,      400, 420, 420);
    init_color(static_ui::C_MAP_GRASS,    0, 255,   0);
    init_color(static_ui::C_MAP_HERO,   555,   0,   0);
    init_color(static_ui::C_MAP_BORDER, 300, 100,   0);

    init_pair(static_ui::C_GRASS_PAIR,
              static_ui::C_MAP_GRASS,
              static_ui::C_BACK);

    init_pair(static_ui::C_HERO_PAIR,
              static_ui::C_MAP_HERO,
              static_ui::C_BACK);

    init_pair(static_ui::C_BORDER_PAIR,
              static_ui::C_BACK,
              static_ui::C_MAP_BORDER);

    init_pair(static_ui::C_DEFAULT_PAIR,
              static_ui::C_FRONT,
              static_ui::C_BACK);

    bkgd(' ' as u32 | COLOR_PAIR(static_ui::C_DEFAULT_PAIR) as u32);
}

/// Anything we need to do to clean up ncurses invokation
fn ncurses_cleanup() {
    endwin();
}

