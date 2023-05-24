use crate::error::ParseError;
use crate::traits::Parse;
use crate::types::axis_value_map::AxisValueMap;

pub struct SegmentMaps {
    /// The number of correspondence pairs for this axis
    pub position_map_count: u16,

    /// The array of axis value map records for this axis
    pub axis_value_map: Vec<AxisValueMap>,
}

impl Parse for SegmentMaps {
    fn parse(data: &[u8]) -> Result<SegmentMaps, ParseError> {
        todo!()
    }
}
