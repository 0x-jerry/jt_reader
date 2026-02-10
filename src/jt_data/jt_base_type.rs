#[derive(Debug)]
pub enum JtBaseType {
    UnknownGraphNode = 255,
    BaseGraphNode = 0,
    GroupGraphNode = 1,
    ShapeGroupNode = 2,
    BaseAttribute = 3,
    ShapeLOD = 4,
    BaseProperty = 5,
    JtObjectReference = 6,
    JTLateLoadedProperty = 8,
    JtBase = 9,
}

impl JtBaseType {
    pub fn new(base_type: u8) -> Self {
        match base_type {
            0 => Self::BaseGraphNode,
            1 => Self::GroupGraphNode,
            2 => Self::ShapeGroupNode,
            3 => Self::BaseAttribute,
            4 => Self::ShapeLOD,
            5 => Self::BaseProperty,
            6 => Self::JtObjectReference,
            8 => Self::JTLateLoadedProperty,
            9 => Self::JtBase,
            _ => Self::UnknownGraphNode,
        }
    }
}