use crate::error::ExportError;

/// Makes the structure able to convert itself to OTTF bytes
pub trait Export {
    /// Exports the structure into OTTF bytes
    fn export(&self) -> Result<Vec<u8>, ExportError>;
}
