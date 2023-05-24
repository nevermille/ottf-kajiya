use crate::error::ExportError;

pub trait Export {
    fn export(&self) -> Result<Vec<u8>, ExportError>;
}
