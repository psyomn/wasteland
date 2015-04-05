use stats::{Stats};

#[derive(Clone)]
#[derive(Debug)]
pub struct Entity {
    name: String,
    stats: Stats,
    friendly: bool,
}

/// An entity is anything that can be some being on the map. Enemies as well as the player are
/// indeed entities.
impl Entity {

    /// Create an entity with default parameters
    pub fn new() -> Entity {
        Entity {
            name:     "defname".to_string(),
            stats:    Stats::new(),
            friendly: false
        }
    }

    /// Set the name of entity
    pub fn name(&mut self, n: String) {
        self.name = n;
    }

    /// Set entity friendlyness
    pub fn friendly(&mut self, f: bool) {
        self.friendly = f;
    }
}
