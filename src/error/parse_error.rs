use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ParseError {
    pub message: String,
    pub kind: String,
}

impl ParseError {
    pub fn from_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
            kind: String::new(),
        }
    }

    pub fn unexpected_end(segment_name: &str) -> Self {
        Self {
            message: format!("Unexpected end for {}", segment_name),
            kind: "Unexpected end".to_string(),
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{{ message: {:?}, kind: {:?} }}",
            &self.message, self.kind
        ))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ParseError {}
