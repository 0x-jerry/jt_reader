use anyhow::Result;

use crate::{
    jt_data::JtData,
    jt_decode::jt_probability_context_table_entry_v2::Int32ProbabilityContextTableEntryV2,
    jt_reader::{BitBufferReader, JtReader},
};

#[derive(Debug, Default)]
pub struct Int32ProbabilityContextV2 {
    probability_context_table_entry_count: u32,
    number_symbol_bits: u32,
    number_occurrence_count_bits: u32,
    number_value_bits: u32,
    min_value: u32,
    table_entries: Vec<Int32ProbabilityContextTableEntryV2>,
}

impl JtData for Int32ProbabilityContextV2 {
    fn read(reader: &mut JtReader) -> Result<Self> {
        let mut result: Self = Default::default();
        let mut bit_buffer = BitBufferReader::new(reader);

        result.probability_context_table_entry_count = bit_buffer.read_u32(16)?;
        result.number_symbol_bits = bit_buffer.read_u32(6)?;
        result.number_occurrence_count_bits = bit_buffer.read_u32(6)?;
        result.number_value_bits = bit_buffer.read_u32(6)?;
        result.min_value = bit_buffer.read_u32(32)?;

        for _ in 0..result.probability_context_table_entry_count {
            let entry = Int32ProbabilityContextTableEntryV2::read(
                &mut bit_buffer,
                result.number_symbol_bits,
                result.number_occurrence_count_bits,
                result.number_value_bits,
            )?;
            result.table_entries.push(entry);
        }

        println!("bit_remaining: {:?}", bit_buffer.bits_remaining);

        Ok(result)
    }
}
