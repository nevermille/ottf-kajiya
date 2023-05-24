use crate::error::ParseError;
use crate::traits::Parse;
use crate::types::TableRecord;
use std::error::Error;

pub struct Kajiya {
    pub sfnt_version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub table_records: Vec<TableRecord>,
}

impl Kajiya {
    fn new_empty() -> Self {
        Self {
            sfnt_version: 0,
            num_tables: 0,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            table_records: vec![],
        }
    }

    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        // Load font file
        let raw_data = std::fs::read(path)?;
        let mut res = Self::new_empty();

        // The header is 12 bytes long
        if raw_data.len() < 12 {
            return Err(Box::new(ParseError::unexpected_end("header")));
        }

        // Data extraction
        res.sfnt_version = u32::from_be_bytes(raw_data[0..4].try_into().unwrap_or_default());
        res.num_tables = u16::from_be_bytes(raw_data[4..6].try_into().unwrap_or_default());
        res.search_range = u16::from_be_bytes(raw_data[6..8].try_into().unwrap_or_default());
        res.entry_selector = u16::from_be_bytes(raw_data[8..10].try_into().unwrap_or_default());
        res.range_shift = u16::from_be_bytes(raw_data[10..12].try_into().unwrap_or_default());

        // The table records segment is 16 bytes per table
        if raw_data.len() < (12 + res.num_tables * 16) as usize {
            return Err(Box::new(ParseError::unexpected_end("header/TableRecords")));
        }

        for i in 0..res.num_tables {
            let first_byte_index = (12 + i * 16) as usize;
            let last_byte_index = (12 + (i + 1) * 16) as usize;

            let table_record = TableRecord::parse(&raw_data[first_byte_index..last_byte_index]);
            res.table_records.push(table_record?);
        }

        Ok(res)
    }
}
