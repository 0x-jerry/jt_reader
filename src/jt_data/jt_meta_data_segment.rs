use anyhow::Result;

use crate::{
    jt_data::{JtData, jt_element::JtElement, jt_logic_element_header::JtLogicElementHeaderZLib},
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtMetaDataSegment {
    pub header: JtLogicElementHeaderZLib,
    pub value: JtElement,
}

impl JtData for JtMetaDataSegment {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        let logic_header = JtLogicElementHeaderZLib::read(reader)?;

        let reader = if logic_header.is_zlib_compressed() {
            let size = size_of_val(&logic_header.compression_algorithm);

            &mut reader.inflate(logic_header.compression_data_length as usize - size)?
        } else {
            reader
        };

        result.value = JtElement::read(reader)?;

        Ok(result)
    }
}
