use anyhow::Result;

use crate::{
    jt_data::{
        JtData, jt_segment_header::JtSegmentHeader,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub struct JtSegment {
    pub header: JtSegmentHeader,
    // pub data: Option<JtSegmentData>,
}

impl JtData for JtSegment {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let header = JtSegmentHeader::read(reader)?;

        Ok(Self { header })
    }
}

impl JtSegment {
    pub fn read_data_as<T: JtData>(&mut self, reader: &mut JtReader) -> Result<T> {
        println!("segment header: {:?}", self.header);

        let result = T::read(reader)?;

        Ok(result)
    }
}
