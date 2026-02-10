use anyhow::Result;

use crate::jt_reader::JtReader;

#[derive(Debug, Default, Clone)]
pub struct JtLogicElementHeaderZLib {
    pub compression_flag: i32,
    pub compression_data_length: i32,
    /// = 1 No compression
    ///
    /// = 2 ZLIB compression
    pub compression_algorithm: u8,
}

impl JtLogicElementHeaderZLib {
    pub fn is_zlib_compressed(&self) -> bool {
        self.compression_flag == 2 && self.compression_algorithm == 2
    }
}

impl JtLogicElementHeaderZLib {
    pub fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();

        result.compression_flag = reader.read_i32()?;
        result.compression_data_length = reader.read_i32()?;
        result.compression_algorithm = reader.read_u8()?;

        return Ok(result);
    }
}
