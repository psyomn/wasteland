extern crate ncurses;

use ncurses::{initscr, printw, refresh, getch, endwin};

mod stats;

fn main() {
    ncurses::initscr();

    ncurses::printw("hello world");
    ncurses::refresh();
    ncurses::getch();

    ncurses::endwin();
}
