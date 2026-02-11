use uuid::Uuid;

use crate::{
    jt_data::{JtData, JtObjectTypeID, jt_base_attribute_data::JtBaseAttributeData},
    jt_data_type::vec4::RGBA,
};

#[derive(Debug, Default)]
pub struct JtMaterialAttributeElement {
    pub base_attribute_data: JtBaseAttributeData,
    pub version: i16,
    pub data_flags: u16,
    pub ambient_color: RGBA,
    pub diffuse_color_and_alpha: RGBA,
    pub specular_color: RGBA,
    pub emission_color: RGBA,
    pub shininess: f32,
    pub reflectivity: i32,
}

impl JtObjectTypeID for JtMaterialAttributeElement {
    const OBJECT_TYPE_ID: Uuid = Uuid::from_fields(
        0x10dd1030,
        0x2ac8,
        0x11d1,
        &[0x9b, 0x6b, 0x00, 0x80, 0xc7, 0xbb, 0x59, 0x97],
    );
}

impl JtData for JtMaterialAttributeElement {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();
        result.base_attribute_data = JtBaseAttributeData::read(reader)?;
        result.version = reader.read_i16()?;
        result.data_flags = reader.read_u16()?;
        result.ambient_color = reader.read_rgba()?;
        result.diffuse_color_and_alpha = reader.read_rgba()?;
        result.specular_color = reader.read_rgba()?;
        result.emission_color = reader.read_rgba()?;
        result.shininess = reader.read_f32()?;

        if result.version == 2 {
            result.reflectivity = reader.read_i32()?;
        }

        Ok(result)
    }
}
