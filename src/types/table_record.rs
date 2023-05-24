use crate::error::{ExportError, ParseError};
use crate::traits::{Export, Parse};

pub struct TableRecord {
    pub tag: String,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

impl Parse for TableRecord {
    fn parse(data: &[u8]) -> Result<TableRecord, ParseError> {
        let tag = String::from_utf8_lossy(&data[0..4]).to_string();
        let checksum = u32::from_be_bytes(data[4..8].try_into().unwrap_or_default());
        let offset = u32::from_be_bytes(data[8..12].try_into().unwrap_or_default());
        let length = u32::from_be_bytes(data[12..16].try_into().unwrap_or_default());

        Ok(Self {
            tag,
            checksum,
            offset,
            length,
        })
    }
}

impl Export for TableRecord {
    fn export(&self) -> Result<Vec<u8>, ExportError> {
        let mut res = Vec::with_capacity(16);

        res.append(&mut self.tag.clone().into_bytes());
        res.append(&mut self.checksum.to_be_bytes().to_vec());
        res.append(&mut self.offset.to_be_bytes().to_vec());
        res.append(&mut self.length.to_be_bytes().to_vec());

        Ok(res)
    }
}
