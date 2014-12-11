use map::{Map};
use ui::ncursable::{NCursable};

/// Drawing logic should be isolated here (which currently isn't).
impl NCursable for Map {
    fn cursify(&self) -> () {
    }
}
