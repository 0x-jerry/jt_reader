use anyhow::bail;
use uuid::Uuid;

use crate::{
    jt_data::{JtData, JtObjectTypeID, jt_date_property_value::JtDatePropertyValue},
    jt_data_type::mbstring::MbString,
};

#[derive(Debug, Default)]
pub struct JtPropertyProxyMetaDataElement {
    pub version: i16,
    pub property_key: MbString,
    pub property_value_type: u8,
    pub string_property_value: MbString,
    pub integer_property_value: i32,
    pub float_property_value: f32,
    pub date_property_value: JtDatePropertyValue,
}

impl JtObjectTypeID for JtPropertyProxyMetaDataElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0xce357247,
        0x38fb,
        0x11d1,
        &[0xa5, 0x6, 0x0, 0x60, 0x97, 0xbd, 0xc6, 0xe1],
    );
}

impl JtData for JtPropertyProxyMetaDataElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result = Self::default();

        result.version = reader.read_i16()?;

        result.property_key = reader.read_mb_string()?;

        if result.property_key.count == 0 {
            return Ok(result);
        }

        result.property_value_type = reader.read_u8()?;

        match result.property_value_type {
            0 => result.string_property_value = reader.read_mb_string()?,
            1 => result.integer_property_value = reader.read_i32()?,
            2 => result.float_property_value = reader.read_f32()?,
            3 => result.date_property_value = JtDatePropertyValue::read(reader)?,
            _ => bail!(
                "Invalid property_value_type: {}",
                result.property_value_type
            ),
        }

        Ok(result)
    }
}
