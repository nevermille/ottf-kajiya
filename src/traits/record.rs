use crate::error::ExportError;
use crate::traits::{Export, Measure};
use std::num::Wrapping;

/// Makes the structure able to export as a TableRecord
pub trait Record: Export + Measure {
    /// Calculates the segment's checksum
    fn checksum(&self) -> Result<u32, ExportError> {
        let mut bytes = self.export()?;
        let mut checksum = Wrapping(0u32);

        while bytes.len() % 4 != 0 {
            bytes.push(0);
        }

        for i in (0..bytes.len()).step_by(4) {
            let value = u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap_or_default());
            checksum += value;
        }

        Ok(checksum.0)
    }
}
