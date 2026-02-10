#[derive(Debug)]
pub enum SegmentType {
    None = 0,
    LogicalSceneGraph = 1,
    JtBRep = 2,
    PMIData = 3,
    MetaData = 4,
    Shape = 6,
    ShapeLod0 = 7,
    ShapeLod1 = 8,
    ShapeLod2 = 9,
    ShapeLod3 = 10,
    ShapeLod4 = 11,
    ShapeLod5 = 12,
    ShapeLod6 = 13,
    ShapeLod7 = 14,
    ShapeLod8 = 15,
    ShapeLod9 = 16,
}

impl SegmentType {
    pub fn new(segment_type: i32) -> Self {
        return match segment_type {
            1 => Self::LogicalSceneGraph,
            2 => Self::JtBRep,
            3 => Self::PMIData,
            4 => Self::MetaData,
            6 => Self::Shape,
            7 => Self::ShapeLod0,
            8 => Self::ShapeLod1,
            9 => Self::ShapeLod2,
            10 => Self::ShapeLod3,
            11 => Self::ShapeLod4,
            12 => Self::ShapeLod5,
            13 => Self::ShapeLod6,
            14 => Self::ShapeLod7,
            15 => Self::ShapeLod8,
            16 => Self::ShapeLod9,
            _ => SegmentType::None,
        };
    }

    pub fn is_shape(&self) -> bool {
        matches!(
            self,
            Self::Shape
                | Self::ShapeLod0
                | Self::ShapeLod1
                | Self::ShapeLod2
                | Self::ShapeLod3
                | Self::ShapeLod4
                | Self::ShapeLod5
                | Self::ShapeLod6
                | Self::ShapeLod7
                | Self::ShapeLod8
                | Self::ShapeLod9
        )
    }

    pub fn is_zlib_applied(&self) -> bool {
        matches!(
            self,
            Self::LogicalSceneGraph | Self::JtBRep | Self::PMIData | Self::MetaData
        )
    }
}
