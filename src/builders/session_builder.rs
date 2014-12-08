use std::io;
use session::{Session};

/// Builds a standard game, with defaul values for the player. This is the non ncurses version of
/// this function.
pub fn build_session() -> Box<Session> {
    let mut s: Box<Session> = box Session::new();
    println!("Name of character?");
    let input: String = io::stdin().read_line().ok().unwrap_or("Heroic Hero".to_string());
    s.set_player_name(input);
    s
}
