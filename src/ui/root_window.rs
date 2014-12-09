extern crate ncurses;
use self::ncurses::*;

pub struct RootWindow {
    height: i16,
    width: i16,
}

/// Standard infor for window
impl RootWindow {
    pub fn new() -> RootWindow { RootWindow { height: 50, width: 80 } }
}
