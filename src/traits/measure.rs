pub trait Measure {
    fn len(&self) -> u32;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
