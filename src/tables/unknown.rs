use crate::error::{ExportError, ParseError};
use crate::traits::{Export, Parse};

/// An unknown table
#[derive(Default, Clone)]
pub struct Unknown {
    /// The raw data
    pub data: Vec<u8>,

    /// The tag, for export reasons
    pub tag: String,
}

impl Parse for Unknown {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            data: data.to_vec(),
            ..Default::default()
        })
    }
}

impl Export for Unknown {
    fn export(&self) -> Result<Vec<u8>, ExportError> {
        Ok(self.data.clone())
    }
}
