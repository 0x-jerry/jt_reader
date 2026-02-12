use std::io::Seek;

use anyhow::Result;

use crate::{
    jt_data::{
        JtData, jt_element::JtElement, jt_lsg_segment::JtLSGSegment,
        jt_meta_data_segment::JtMetaDataSegment, jt_segment_header::JtSegmentHeader,
        jt_segment_type::SegmentType,
    },
    jt_reader::JtReader,
};

#[derive(Debug)]
pub enum JtSegementValue {
    LSG(JtLSGSegment),
    MetaData(JtMetaDataSegment),
    Element(JtElement),
}

impl JtSegementValue {
    pub fn read(reader: &mut JtReader, segment_type: &SegmentType) -> Result<Self> {
        let value = match segment_type {
            SegmentType::LogicalSceneGraph => Self::LSG(JtLSGSegment::read(reader)?),
            SegmentType::ShapeLod0
            | SegmentType::ShapeLod1
            | SegmentType::ShapeLod2
            | SegmentType::ShapeLod3
            | SegmentType::ShapeLod4
            | SegmentType::ShapeLod5
            | SegmentType::ShapeLod6
            | SegmentType::ShapeLod7
            | SegmentType::ShapeLod8
            | SegmentType::ShapeLod9 => Self::Element(JtElement::read(reader)?),
            SegmentType::MetaData => Self::MetaData(JtMetaDataSegment::read(reader)?),
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
        let begin_pos = reader.reader.stream_position()?;

        let header = JtSegmentHeader::read(reader)?;

        log::info!("segment header: {:?}, begin: {}", header, begin_pos);

        let value = JtSegementValue::read(reader, &header.s_type)?;

        let end_pos = reader.reader.stream_position()?;

        let read_len = end_pos - begin_pos;
        let alignment_pad_len = header.length as i64 - read_len as i64;
        log::debug!(
            "segment length: {}, read length: {}, alignment pad length: {}",
            header.length,
            read_len,
            alignment_pad_len
        );

        Ok(Self { header, value })
    }
}
