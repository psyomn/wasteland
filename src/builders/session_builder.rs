use session::{Session};
use map::{Map};

/// Builds a standard game, with defaul values for the player. This is the non ncurses version of
/// this function.
pub fn build_session(player_name: String) -> Box<Session> {
    let m: Box<Map> = box Map::new(30,30);
    let mut s: Box<Session> = box Session::new(m);
    s.set_player_name(player_name);
    s
}
