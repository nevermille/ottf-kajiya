use crate::error::{ExportError, ParseError};
use crate::traits::{Export, Measure, Parse};
use crate::types::axis_value_map::AxisValueMap;

pub struct SegmentMaps {
    /// The array of axis value map records for this axis
    pub axis_value_map: Vec<AxisValueMap>,
}

impl SegmentMaps {
    fn new_empty() -> Self {
        Self {
            axis_value_map: Vec::new(),
        }
    }

    /// Returns the length in bytes of the segment
    pub fn len(&self) -> usize {
        2 + self.axis_value_map.len() * 4
    }
}

impl Measure for SegmentMaps {
    fn len(&self) -> u32 {
        (2 + self.axis_value_map.len() * 4) as u32
    }
}

impl Parse for SegmentMaps {
    fn parse(data: &[u8]) -> Result<SegmentMaps, ParseError> {
        let mut res = Self::new_empty();

        if data.len() < 2 {
            return Err(ParseError::unexpected_end("avar/SegmentMaps"));
        }

        let position_map_count = u16::from_be_bytes(data[0..2].try_into().unwrap_or_default());
        let expected_minimum_bytes = (2 + position_map_count * 4) as usize;

        if data.len() < expected_minimum_bytes {
            return Err(ParseError::unexpected_end("avar/SegmentMaps/AxisValueMap"));
        }

        for i in 0..position_map_count {
            let first_byte = (2 + i * 4) as usize;
            let last_byte = (2 + (i + 1) * 4) as usize;

            res.axis_value_map
                .push(AxisValueMap::parse(&data[first_byte..last_byte])?);
        }

        Ok(res)
    }
}

impl Export for SegmentMaps {
    fn export(&self) -> Result<Vec<u8>, ExportError> {
        let count = self.axis_value_map.len() as u16;
        let mut res = Vec::new();

        // Export positionMapCount
        res.append(&mut count.to_be_bytes().to_vec());

        // Export axisValueMaps
        for i in 0..count {
            res.append(&mut self.axis_value_map[i as usize].export()?);
        }

        Ok(res)
    }
}
