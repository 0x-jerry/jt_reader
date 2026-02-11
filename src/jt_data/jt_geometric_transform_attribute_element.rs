use uuid::Uuid;

use crate::jt_data::{JtData, JtObjectTypeID, jt_base_attribute_data::JtBaseAttributeData};

#[derive(Debug, Default)]
pub struct JtGeometricTransformAttributeElement {
    pub base_attribute_data: JtBaseAttributeData,
    pub version: i16,
    pub stored_value_mask: u16,
    pub element_values: [f64; 16],
}

impl JtObjectTypeID for JtGeometricTransformAttributeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd1083,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtGeometricTransformAttributeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();

        result.base_attribute_data = JtBaseAttributeData::read(reader)?;
        result.version = reader.read_i16()?;

        let mut stored_value_mask = reader.read_u16()?;
        result.stored_value_mask = stored_value_mask;

        for i in 0..16 {
            if (stored_value_mask & 0x8000) != 0 {
                // https://github.com/cbsghost/oce-jt/blob/a7ea581f0dba325c649bab7fe1bc68ca5e85b946/TKJT/src/JtAttribute/JtAttribute_GeometricTransform.cxx#L61-L65
                result.element_values[i] = reader.read_f64()?;
            }
            stored_value_mask = stored_value_mask << 1;
        }

        Ok(result)
    }
}
