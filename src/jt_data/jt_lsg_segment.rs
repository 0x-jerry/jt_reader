use anyhow::Result;

use crate::{
    jt_data::{
        JtData, jt_element::JtElement, jt_logic_element_header::JtLogicElementHeaderZLib,
        jt_property_table_data::JtPropertyTableData,
    },
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtLSGSegment {
    pub header: JtLogicElementHeaderZLib,

    pub elements: Vec<JtElement>,
    pub property_table: JtPropertyTableData,
}

impl JtLSGSegment {
    /// Return true if the end of elements marker is read.
    fn read_element(&mut self, reader: &mut JtReader) -> Result<bool> {
        let element = JtElement::read(reader)?;

        if element.is_end_marker_element() {
            return Ok(true);
        }

        self.elements.push(element);

        Ok(false)
    }
}

impl JtData for JtLSGSegment {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        let logic_header = JtLogicElementHeaderZLib::read(reader)?;

        let reader = if logic_header.is_zlib_compressed() {
            let size = size_of_val(&logic_header.compression_algorithm);

            &mut reader.inflate(logic_header.compression_data_length as usize - size)?
        } else {
            reader
        };

        // Read graph elements.
        while !result.read_element(reader)? {}

        // Read property atom elements.
        while !result.read_element(reader)? {}

        result.property_table = JtPropertyTableData::read(reader)?;

        Ok(result)
    }
}
