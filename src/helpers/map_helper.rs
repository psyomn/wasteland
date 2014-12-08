use std::rand;
use std::rand::Rng;

use game_entity::{Entity};
use map::{Map};

/// Poppulate map, with random entity structs. n is for number of entities
pub fn poppulate_with_entities(m: &mut Map, n: u16) {
    let h : uint = m.height() as uint;
    let w : uint = m.width() as uint;
    let mut r = rand::task_rng();
    let rmx = |max| -> uint { r.gen::<uint>() % max };

    for _ in range(0u16, n) {
        /* n times */
        let (x, y) = (rmx(w), rmx(h));
        let e: Box<Entity> = box Entity::new();
        m.place_entity_at(x, y, e);
    }
}
