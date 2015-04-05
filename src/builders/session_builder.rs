use session::{Session};
use map::{Map};
use game_entity::{Entity};

/// Builds a standard game, with defaul values for the player. This is the non ncurses version of
/// this function.
pub fn build_session<'a>(player_name: String) -> Box<Session<'a>> {
    let m: Box<&mut Map>    = Box::new(&mut Map::new(78,30));
    let e: Box<Entity>      = Box::new(Entity::new());
    m.place_player(e);

    let mut s = Session::new(m);
    s
}
