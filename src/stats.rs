struct Stats {
    strength:  i32,
    defense:   i32,
    magic:     i32,
    endurance: i32,
    hitpoints: i32,
}

impl Stats {

    pub fn new() -> Stats {
        Stats {
            strength: 1i32,
            defense: 1i32,
            magic: 1i32,
            endurance: 10i32,
            hitpoints: 10i32, }
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
            self.hitpoints = if -d > self.hitpoints { 0 } else { self.hitpoints + d };
        }
    }

}

#[test]
fn test_simple_damage() {
    let mut s: Stats = Stats::new();
    s.apply(-2);
    assert!(8i32 == s.hitpoints);
}

#[test]
fn test_overkill_damage() {
    let mut s: Stats = Stats::new();
    s.apply(-9000);
    assert!(0i32 == s.hitpoints);
}

#[test]
fn test_damage_and_heal() {
    let mut s: Stats = Stats::new();
    s.apply(-3);
    s.apply(2);
    assert!(s.endurance - 3 + 2 == s.hitpoints);
}

#[test]
fn test_overheal_damage() {
    let mut s: Stats = Stats::new();
    s.apply(9000);
    assert!(s.endurance == s.hitpoints);
}

