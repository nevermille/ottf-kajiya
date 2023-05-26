use crate::error::ExportError;
use crate::error::ParseError;
use crate::traits::Export;
use crate::traits::Parse;

/// Axis value map records for an axis
#[derive(Default, Clone)]
pub struct AxisValueMap {
    /// A normalized coordinate value obtained using default normalization
    ///
    /// Warning: this u16 is a F2DOT14
    pub from_coordinate: u16,

    /// The modified, normalized coordinate value
    ///
    /// Warning: this u16 is a F2DOT14
    pub to_coordinate: u16,
}

impl Parse for AxisValueMap {
    fn parse(data: &[u8]) -> Result<AxisValueMap, ParseError> {
        if data.len() < 4 {
            return Err(ParseError::unexpected_end("avar/SegmentMaps/AxisValueMap"));
        }

        let from_coordinate = u16::from_be_bytes(data[0..2].try_into().unwrap_or_default());
        let to_coordinate = u16::from_be_bytes(data[2..4].try_into().unwrap_or_default());

        Ok(Self {
            from_coordinate,
            to_coordinate,
        })
    }
}

impl Export for AxisValueMap {
    fn export(&self) -> Result<Vec<u8>, ExportError> {
        let mut res = Vec::with_capacity(4);

        // Export fromCoordinate and toCoordinate
        res.append(&mut self.from_coordinate.to_be_bytes().to_vec());
        res.append(&mut self.to_coordinate.to_be_bytes().to_vec());

        Ok(res)
    }
}
