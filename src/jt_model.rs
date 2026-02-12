use std::{fs::File, io::SeekFrom};

use anyhow::{Result, bail};

use crate::{
    jt_data::{
        JtData,
        jt_file_header::JtFileHeader,
        jt_element::{JtElement, JtElementValue},
        jt_lsg_segment::JtLSGSegment,
        jt_segment::JtSegment,
        jt_toc_entity::JtTocEntry,
    },
    jt_reader::JtReader,
};

pub struct JtModel {
    reader: JtReader,
    pub header: JtFileHeader,
    pub toc: Vec<JtTocEntry>,
}

impl JtModel {
    pub fn new(file: File) -> Result<Self> {
        let mut reader = JtReader::from_file(file)?;
        let header = JtFileHeader::read(&mut reader)?;
        reader.set_byte_order(header.byte_order);

        let mut model = Self {
            reader,
            header,
            toc: Vec::new(),
        };

        model.parse_toc()?;

        Ok(model)
    }

    fn parse_toc(&mut self) -> Result<()> {
        let offset = self.header.toc_offset;

        self.reader.seek(SeekFrom::Start(offset as u64))?;

        let entry_count = self.reader.read_i32()?;

        for _ in 0..entry_count {
            let entity = JtTocEntry::read(&mut self.reader)?;

            self.toc.push(entity);
        }

        Ok(())
    }

    pub fn seek(&mut self, offset: u64) -> Result<()> {
        self.reader.seek(SeekFrom::Start(offset))?;
        Ok(())
    }

    pub fn extract_meshes(&mut self) -> Result<Vec<crate::mesh::Mesh>> {
        let meshes = Vec::new();

        let toc = self.toc.clone();

        for entry in toc {
            self.seek(entry.offset as u64)?;
            let _segment = JtSegment::read(&mut self.reader)?;
        }

        Ok(meshes)
    }
}
