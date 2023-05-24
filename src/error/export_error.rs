use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ExportError {
    message: String,
}

impl ExportError {
    pub fn from_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Debug for ExportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{ message: {} }}", &self.message))
    }
}

impl Display for ExportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ExportError {}
