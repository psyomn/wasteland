use stats::{Stats};

#[derive(Clone)]
pub struct Entity {
    name: String,
    stats: Stats,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            name: "defname".to_string(),
            stats: Stats::new() }
    }

    pub fn name(&mut self, n: String) {
        self.name = n;
    }
}
