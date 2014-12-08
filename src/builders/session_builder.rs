use std::io;
use session::{Session};

/// Builds a standard game, with defaul values for the player. This is the non ncurses version of
/// this function.
pub fn build_session(player_name: String) -> Box<Session> {
    let mut s: Box<Session> = box Session::new();
    s.set_player_name(player_name);
    s
}
