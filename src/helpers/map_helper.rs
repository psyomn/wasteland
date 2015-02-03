use std::rand;
use std::rand::Rng;

use game_entity::{Entity};
use map::{Map};

/// Poppulate map, with random entity structs. n is for number of entities
pub fn poppulate_with_entities(m: &mut Map, n: u16) {
    let h : u32 = m.height() as u32;
    let w : u32 = m.width() as u32;
    let mut r = rand::thread_rng();
    let rmx = |&:max:u32| -> u32 {
        let mut r = rand::thread_rng();
        r.gen::<u32>() % max };

    for _ in range(0u16, n) {
        /* n times */
        let x = rmx(w);
        let y = rmx(h);
        let e: Box<Entity> = Box::new(Entity::new());
        m.place_entity_at(x as uint, y as uint, e );
    }
}
