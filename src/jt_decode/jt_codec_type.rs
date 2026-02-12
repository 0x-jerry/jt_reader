use anyhow::{Result, bail};

#[derive(Debug, Eq, PartialEq)]
pub enum JtCodecType {
    Null = 0,
    Bitlength = 1,
    Arithmetic = 3,
    Chopper = 4,
}

impl JtCodecType {
    pub fn from(value: u8) -> Result<Self> {
        let result = match value {
            0 => JtCodecType::Null,
            1 => JtCodecType::Bitlength,
            3 => JtCodecType::Arithmetic,
            4 => JtCodecType::Chopper,
            _ => bail!("Unsuppoert JtCodecType value {}", value),
        };

        Ok(result)
    }
}
