use anyhow::Result;

use crate::jt_reader::BitBufferReader;

#[derive(Debug, Default)]
pub struct Int32ProbabilityContextTableEntryV2 {
    pub symbol: i32,
    pub occurrence_count: u32,
    pub associated_value: u32,
}

impl Int32ProbabilityContextTableEntryV2 {
    pub fn read(
        bit_buffer: &mut BitBufferReader,
        number_symbol_bits: u32,
        number_occurrence_count_bits: u32,
        number_value_bits: u32,
    ) -> Result<Self> {
        let mut result: Self = Default::default();

        result.symbol = bit_buffer.read_u32(number_symbol_bits as usize)? as i32 - 2;
        result.occurrence_count = bit_buffer.read_u32(number_occurrence_count_bits as usize)?;
        result.associated_value = bit_buffer.read_u32(number_value_bits as usize)?;

        Ok(result)
    }
}
