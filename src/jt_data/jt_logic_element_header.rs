use anyhow::Result;

use crate::jt_reader::JtReader;

#[derive(Debug, Default, Clone)]
pub struct JtLogicElementHeader {
    pub compression_flag: i32,
    pub compression_data_length: i32,
    /// = 1 No compression
    ///
    /// = 2 ZLIB compression
    pub compression_algorithm: u8,
    pub element_length: i32,
}

impl JtLogicElementHeader {
    pub fn is_zlib_compressed(&self) -> bool {
        self.compression_flag == 2 && self.compression_algorithm == 2
    }
}

impl JtLogicElementHeader {
    pub fn read(reader: &mut JtReader, is_zlib_applied: bool) -> Result<Self> {
        let mut result: Self = Default::default();

        if is_zlib_applied {
            result.compression_flag = reader.read_i32()?;
            result.compression_data_length = reader.read_i32()?;
            result.compression_algorithm = reader.read_u8()?;
        } else {
            result.element_length = reader.read_i32()?;
        }

        return Ok(result);
    }

    pub fn read_length(&mut self, reader: &mut JtReader) -> Result<()> {
        self.element_length = reader.read_i32()?;

        Ok(())
    }
}
