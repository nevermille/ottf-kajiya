use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn from_message(message: &str) -> Self {
        Self{ message: message.to_string() }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{ message: {} }}", &self.message))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ParseError {}