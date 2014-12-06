use std::rand;
use std::rand::Rng;

use game_entity::{Entity};
use map::{Map};

/// Poppulate map, with random entity structs. n is for number of entities
pub fn poppulate_with_entities(m: &mut Map, n: u16) {
    let h = m.height();
    let w = m.width();
    let mut r = rand::task_rng();
    let rmx = |max| -> u32 { r.gen::<u32>() % max };

    for times in range(0u16, n) {
        /* n times */
        let (x, y) = (rmx(w), rmx(h));
        let mut e: Entity = Entity::new();
        m.place_entity_at(x, y, &mut e);
    }
}
