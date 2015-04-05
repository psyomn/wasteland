use map::{Map};
use session::{Session};
use stats::{Stats};
use builders;

/// Make the game data / session
pub fn make_game_data<'a>() -> Box<Session<'a>> {
    let s: Stats = Stats::new();
    let mut m: Box<Map> = Box::new(Map::new(78u32, 30u32));
    let mut session = builders::session_builder::build_session("hiro".to_string());
    m.name("The badlands".to_string());
    m.randomize();
    session.set_current_map(m);
    session
}
