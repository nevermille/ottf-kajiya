use crate::types::SegmentMaps;

pub struct Avar {
    pub major_version: u16,
    pub minor_version: u16,
    pub reserved: u16,
    pub axis_count: u16,
    pub segment_maps: Vec<SegmentMaps>,
}
