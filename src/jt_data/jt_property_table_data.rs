use std::collections::HashMap;

use crate::jt_data::JtData;

#[derive(Debug, Default)]
pub struct JtElementPropertyTableData {
    /// pairs<(key_property_atom_object_id, value_property_atom_object_id)>
    pub pairs: Vec<(i32, i32)>,
}

impl JtData for JtElementPropertyTableData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();

        loop {
            let key = reader.read_i32()?;
            if key == 0 {
                break;
            }

            let value = reader.read_i32()?;
            result.pairs.push((key, value));
        }

        Ok(result)
    }
}

#[derive(Debug, Default)]
pub struct JtPropertyTableData {
    pub version: i16,
    pub element_property_table_count: i32,
    // Element property id => data
    pub element_property_table: HashMap<i32, JtElementPropertyTableData>,
}

impl JtData for JtPropertyTableData {
    fn read(reader: &mut crate::jt_reader::JtReader) -> anyhow::Result<Self> {
        let mut result: Self = Default::default();

        result.version = reader.read_i16()?;
        result.element_property_table_count = reader.read_i32()?;

        for _ in 0..result.element_property_table_count {
            let element_property_id = reader.read_i32()?;
            let element_property_table_data = JtElementPropertyTableData::read(reader)?;

            result
                .element_property_table
                .insert(element_property_id, element_property_table_data);
        }

        Ok(result)
    }
}
