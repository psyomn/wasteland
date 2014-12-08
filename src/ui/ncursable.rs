pub trait NCursable {
    /// Common trait for things that use ncurses in order to print things
    /// on screen
    fn cursify(&self) -> ();
}
