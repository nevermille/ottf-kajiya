use crate::error::ExportError;
use crate::traits::Export;

pub struct Unknown {
    data: Vec<u8>,
}

impl Export for Unknown {
    fn export(&self) -> Result<Vec<u8>, ExportError> {
        Ok(self.data.clone())
    }
}
