use anyhow::{Result, bail};

use crate::{
    jt_data::{
        JtCompressedSegment, JtData, jt_element_header::JtElementHeader,
        jt_graph_element::JtGraphElement, jt_logic_element_header::JtLogicElementHeaderZLib,
    },
    jt_reader::JtReader,
};

#[derive(Debug, Default)]
pub struct JtLSGSegment {
    pub header: JtLogicElementHeaderZLib,

    pub graph_elements: Vec<JtGraphElement>,
    // pub property_atom_elements: Vec<JtPropertyAtomElement>,
    // pub property_table: JtPropertyTable,
}

impl JtCompressedSegment for JtLSGSegment {
    const IS_COMPRESSED: bool = true;
}

impl JtLSGSegment {
    /// Return true if the end of elements marker is read.
    fn read_graph_element(&mut self, reader: &mut JtReader) -> Result<bool> {
        let element_length = reader.read_i32()?;
        let element_header = JtElementHeader::read(reader)?;
        println!(
            "element length: {}, element header: {:?}",
            element_length, element_header
        );

        let value = JtGraphElement::read(reader, element_header.object_type_id)?;

        println!("graph element: {:#?}", value);
        if value.is_end() {
            return Ok(true);
        }

        if value.is_none() {
            bail!("Unimplemented object type: {:?}", element_header);
        }

        self.graph_elements.push(value);

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

        println!("logic header data: {:?}", logic_header);

        while !result.read_graph_element(reader)? {}

        // loop {
        //     let object_type_id = reader.read_uuid()?;
        //     let graph_element = JtGraphElement::read(reader, object_type_id)?;
        //     graph_elements.push(graph_element);
        //     if graph_element == JtGraphElement::EndOfElements {
        //         break;
        //     }
        // }

        Ok(result)
    }
}
