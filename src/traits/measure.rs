/// Adds function for measurements
pub trait Measure {
    /// Returns the segment's length in bytes
    fn len(&self) -> u32;

    /// Check if segment is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
