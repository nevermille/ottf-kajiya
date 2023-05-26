use crate::error::ParseError;
use crate::tables::Tables;
use crate::traits::Parse;
use crate::types::TableRecord;
use std::error::Error;

/// The main structure, representing an OTTF font file
#[derive(Default, Clone)]
pub struct Kajiya {
    /// 0x00010000 or 0x4F54544F ('OTTO')
    ///
    /// OpenType fonts that contain TrueType outlines should use the value$
    /// of 0x00010000 for the sfntVersion. OpenType fonts containing CFF data
    /// (version 1 or 2) should use 0x4F54544F ('OTTO', when re-interpreted as a Tag)
    /// for sfntVersion
    pub sfnt_version: u32,

    /// The font's tables
    pub tables: Tables,
}

impl Kajiya {
    /// Parses the entire font file
    ///
    /// # Parameters
    ///
    /// * `path`: The font's path
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        // Load font file
        let raw_data = std::fs::read(path)?;

        // Parse file
        Ok(Self::parse(&raw_data)?)
    }
}

impl Parse for Kajiya {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        let mut res = Self::default();

        // The header is 12 bytes long
        if data.len() < 12 {
            return Err(ParseError::unexpected_end("header"));
        }

        // Data extraction, we don't need searchRange, entrySelector and rangeShift
        res.sfnt_version = u32::from_be_bytes(data[0..4].try_into().unwrap_or_default());
        let num_tables = u16::from_be_bytes(data[4..6].try_into().unwrap_or_default());

        // The table records segment is 16 bytes per table
        if data.len() < (12 + num_tables * 16) as usize {
            return Err(ParseError::unexpected_end("header/TableRecords"));
        }

        // Table records extraction
        for i in 0..num_tables {
            let first_byte_index = (12 + i * 16) as usize;
            let last_byte_index = (12 + (i + 1) * 16) as usize;

            let table_record = TableRecord::parse(&data[first_byte_index..last_byte_index])?;
            let first_byte = table_record.offset as usize;
            let last_byte = first_byte + table_record.length as usize;

            res.tables
                .add_table(&table_record.tag, &data[first_byte..last_byte])?;
        }

        Ok(res)
    }
}
