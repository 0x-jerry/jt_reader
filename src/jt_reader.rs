use anyhow::{Result, bail};
use byteorder::ReadBytesExt;
use flate2::read::ZlibDecoder;
use log::info;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}
pub struct JtReader {
    pub reader: BufReader<Cursor<Vec<u8>>>,
    pub byte_order: ByteOrder,
}

impl JtReader {
    pub fn new(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self {
            reader: BufReader::new(Cursor::new(bytes)),
            byte_order: ByteOrder::LittleEndian,
        })
    }

    pub fn from_file(file: File) -> Result<Self> {
        let mut buf = Vec::new();
        let mut file = file;
        file.read_to_end(&mut buf)?;

        Self::new(buf)
    }

    pub fn inflate(&mut self, length: usize) -> Result<Self> {
        let mut buf = vec![0u8; length];
        self.read_exact(&mut buf)?;

        let mut decoder = ZlibDecoder::new(&buf[0..]);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        let mut reader = Self::new(decompressed_data)?;
        reader.set_byte_order(self.byte_order);

        Ok(reader)
    }

    pub fn set_byte_order(&mut self, byte_order: ByteOrder) {
        self.byte_order = byte_order;
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.reader.read_u8().map_err(Into::into)
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.reader.read_i8().map_err(Into::into)
    }

    /// The GUID type is a 16 byte (128-bit) number. GUID is
    /// stored/written to the JT file using a four-byte word (U32), 2
    /// two-byte words (U16), and 8 one-byte words (U8) such as:
    /// {3F2504E0-4F89-11D3-9A-0C-03-05-E8-2C-33-01}
    /// In the JT format GUIDs are used as unique identifiers (e.g.
    /// Data Segment ID, Object Type ID, etc.)
    pub fn read_guid(&mut self) -> Result<Uuid> {
        let mut guid_bytes = [0u8; 16];

        self.reader.read_exact(&mut guid_bytes)?;

        if self.byte_order == ByteOrder::BigEndian {
            return Ok(Uuid::from_bytes(guid_bytes));
        }

        Ok(Uuid::from_bytes_le(guid_bytes))
    }

    pub fn read_string(&mut self, byte_size: usize) -> Result<String> {
        let mut buf = vec![0u8; byte_size];

        self.reader.read_exact(&mut buf)?;

        let str = String::from_utf8_lossy(&buf);
        Ok(str.to_string())
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.reader.seek(pos).map_err(Into::into)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.reader.read_exact(buf).map_err(Into::into)
    }
}

pub struct BitBufferReader<'a> {
    pub reader: &'a mut JtReader,
    bit_buf: u64,
    pub bits_remaining: usize,
}

impl<'a> BitBufferReader<'a> {
    pub fn new(reader: &'a mut JtReader) -> Self {
        Self {
            reader,
            bit_buf: 0,
            bits_remaining: 0,
        }
    }

    /// Obtain a bit buffer containing the needed number of bits.
    pub fn read_u32(&mut self, the_bits_count: usize) -> Result<u32> {
        if the_bits_count == 0 {
            return Ok(0);
        }

        if the_bits_count > 32 {
            bail!("read_u32: the_bits_count must be less than or equal to 32, {}", the_bits_count);
        }

        while self.bits_remaining < the_bits_count {
            let size = 32;
            let next_word = self.reader.read_u32()?;
            self.bit_buf <<= size;
            self.bit_buf |= next_word as u64;
            self.bits_remaining += size;
        }

        let head = self.bits_remaining;
        let tail = head - the_bits_count;
        let suffix = 32 - the_bits_count;

        let result = ((self.bit_buf >> tail) << suffix) as u32;

        let result = result >> suffix;
        // println!("result {}", result);
        // bail!("test");

        self.bits_remaining -= the_bits_count;

        Ok(result)
    }

    pub fn clear(&mut self) {
        self.bit_buf = 0;
        self.bits_remaining = 0;
    }
}
