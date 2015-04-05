use std::rand;
use std::rand::Rng;

#[derive(Clone)]
#[derive(Debug)]
pub struct Stats {
    strength:     i32,
    defense:      i32,
    magic:        i32,
    intelligence: i32,
    endurance:    i32,
    hitpoints:    i32,
}

impl Stats {

    pub fn new() -> Stats {
        Stats {
            strength:     1i32,
            defense:      1i32,
            magic:        1i32,
            intelligence: 1i32,
            endurance:    10i32,
            hitpoints:    10i32, }
    }

    /// Safe way of receiving damage or healing. Pass positive numbers to heal. Negative to damage
    /// the player.
    pub fn apply(&mut self, d: i32) {
        if d >= 0 {
            /* Heal */
            if self.hitpoints + d > self.endurance {
                self.hitpoints = self.endurance;
            }
            else {
                self.hitpoints += d;
            }
        }
        else {
            /* Damage */
            let with_defense: i32 = self.defense - d;
            let all: i32 = if with_defense < 0 { 0 } else { with_defense };
            self.hitpoints = if -d > self.hitpoints { 0 } else { self.hitpoints + with_defense };
        }
    }

    /// Attack, and return something with strength with a +- deviance of 10%
    pub fn attack(&self) -> i32 {
        let s: f32      = self.strength as f32;
        let mut r       = rand::thread_rng();
        let sign: bool  = r.gen::<bool>();
        let result: f32 = s + if sign {1.0} else {-1.0} * s * 0.10f32;
        result as i32
    }

}

