use crate::error::ParseError;

pub trait Parse: Sized {
    fn parse(data: &[u8]) -> Result<Self, ParseError>;
}
