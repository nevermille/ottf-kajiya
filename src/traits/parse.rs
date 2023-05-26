use crate::error::ParseError;

/// Makes the structure able to parse an OTTF segment
pub trait Parse: Sized {
    /// Parses the bytes slice
    ///
    /// # Parameters
    ///
    /// * `data`: The slice
    fn parse(data: &[u8]) -> Result<Self, ParseError>;
}
