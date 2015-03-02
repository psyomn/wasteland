use session::{Session};
use map::{Map};

/// Builds a standard game, with defaul values for the player. This is the non ncurses version of
/// this function.
pub fn build_session(player_name: String) -> Box<Session> {
    let m: Box<Map>         = Box::new(Map::new(78,30));
    let mut s: Box<Session> = Box::new(Session::new(m));
    s.set_player_name(player_name);
    s
}
