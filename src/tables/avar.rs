use crate::error::ParseError;
use crate::traits::Parse;
use crate::types::SegmentMaps;

pub struct Avar {
    pub major_version: u16,
    pub minor_version: u16,
    pub reserved: u16,
    pub segment_maps: Vec<SegmentMaps>,
}

impl Avar {
    fn new_empty() -> Self {
        Self {
            major_version: 0,
            minor_version: 0,
            reserved: 0,
            segment_maps: vec![],
        }
    }
}

impl Parse for Avar {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        let mut res = Self::new_empty();

        // We need at least 8 bytes
        if data.len() < 8 {
            return Err(ParseError::unexpected_end("avar"));
        }

        // Extraction of simple data
        res.major_version = u16::from_be_bytes(data[0..2].try_into().unwrap_or_default());
        res.minor_version = u16::from_be_bytes(data[2..4].try_into().unwrap_or_default());
        res.reserved = u16::from_be_bytes(data[4..6].try_into().unwrap_or_default());

        // To extract SegmentMaps values, we need an offset because records' lengths are
        // variable
        let axis_count = u16::from_be_bytes(data[6..8].try_into().unwrap_or_default());
        let mut offset = 8;

        for _ in 0..axis_count {
            // We check if slice won't panic
            if offset >= data.len() {
                return Err(ParseError::unexpected_end("avar/SegmentMaps"));
            }

            // We parse each SegmentMaps and we update the offset
            let segment_maps = SegmentMaps::parse(&data[offset..])?;
            offset += segment_maps.len();
            res.segment_maps.push(segment_maps);
        }

        Ok(res)
    }
}

// Well... we need to do a manual test since I'm unable to find a font
// containing an `avar` table
#[cfg(test)]
mod test {
    use crate::tables::Avar;
    use crate::traits::Parse;

    #[test]
    fn parse() {
        let data: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x02, 0x2c, 0x9e, 0x65, 0xa5,
            0x00, 0x85, 0x36, 0x89, 0x00, 0x03, 0xa3, 0xff, 0x87, 0xd2, 0x64, 0x78, 0xab, 0x54,
            0x37, 0x53, 0x83, 0xe3, 0x00, 0x01, 0x57, 0x99, 0x79, 0x26,
        ];
        let avar = Avar::parse(&data).unwrap();

        assert_eq!(avar.major_version, 1);
        assert_eq!(avar.minor_version, 0);
        assert_eq!(avar.reserved, 0);
        assert_eq!(avar.segment_maps.len(), 3);

        let segment_map_0 = &avar.segment_maps[0];
        assert_eq!(segment_map_0.axis_value_map.len(), 2);
        assert_eq!(segment_map_0.axis_value_map[0].from_coordinate, 0x2c9e);
        assert_eq!(segment_map_0.axis_value_map[0].to_coordinate, 0x65a5);
        assert_eq!(segment_map_0.axis_value_map[1].from_coordinate, 0x0085);
        assert_eq!(segment_map_0.axis_value_map[1].to_coordinate, 0x3689);

        let segment_map_2 = &avar.segment_maps[2];
        assert_eq!(segment_map_2.axis_value_map.len(), 1);
        assert_eq!(segment_map_2.axis_value_map[0].from_coordinate, 0x5799);
        assert_eq!(segment_map_2.axis_value_map[0].to_coordinate, 0x7926);
    }
}
