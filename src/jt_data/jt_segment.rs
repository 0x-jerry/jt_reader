use std::io::Seek;

use anyhow::Result;

use crate::{
    jt_data::{
        JtData, jt_element::JtElement, jt_lsg_segment::JtLSGSegment,
        jt_segment_header::JtSegmentHeader, jt_segment_type::SegmentType,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub enum JtSegementValue {
    LSG(JtLSGSegment),
    Element(JtElement),
}

impl JtSegementValue {
    pub fn read(reader: &mut JtReader, segment_type: &SegmentType) -> Result<Self> {
        let value = match segment_type {
            SegmentType::LogicalSceneGraph => Self::LSG(JtLSGSegment::read(reader)?),
            SegmentType::Shape
            | SegmentType::ShapeLod0
            | SegmentType::ShapeLod1
            | SegmentType::ShapeLod2
            | SegmentType::ShapeLod3
            | SegmentType::ShapeLod4
            | SegmentType::ShapeLod5
            | SegmentType::ShapeLod6
            | SegmentType::ShapeLod7
            | SegmentType::ShapeLod8
            | SegmentType::ShapeLod9 => Self::Element(JtElement::read(reader)?),
            _ => anyhow::bail!("Unsupported segment type: {:?}", segment_type),
        };

        Ok(value)
    }
}

#[derive(Debug)]
pub struct JtSegment {
    pub header: JtSegmentHeader,
    pub value: JtSegementValue,
}

impl JtData for JtSegment {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let header = JtSegmentHeader::read(reader)?;
        let begin = reader.reader.stream_position()?;

        log::info!("segment header: {:?}, begin: {}", header, begin);

        let value = JtSegementValue::read(reader, &header.s_type)?;

        let end = reader.reader.stream_position()?;

        let length = end - begin + (16 + 4 + 4);
        log::debug!(
            "end: {}, segment length: {}, read length: {}",
            end,
            header.length,
            length
        );

        Ok(Self { header, value })
    }
}
