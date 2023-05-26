use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

/// An error during parsing
pub struct ParseError {
    /// The error message
    pub message: String,

    /// The error type
    pub kind: String,
}

impl ParseError {
    /// Creates an error from the message
    ///
    /// # Parameters
    ///
    /// * `message`: The error message
    pub fn from_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
            kind: "Error".to_string(),
        }
    }

    /// Error from a too short segment
    ///
    /// # Parameters
    ///
    /// * `segment_name`: The segment name
    pub fn unexpected_end(segment_name: &str) -> Self {
        Self {
            message: format!("Unexpected end for {}", segment_name),
            kind: "UnexpectedEnd".to_string(),
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
