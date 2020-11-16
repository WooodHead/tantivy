use std::io;
use super::BLOCK_LEN;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct BlockReader<'a> {
    buffer: Vec<u8>,
    reader: Box<dyn io::Read + 'a>,
    offset: usize,
}

impl<'a> BlockReader<'a> {

    pub fn new(reader: Box<dyn io::Read + 'a>) -> BlockReader<'a> {
        BlockReader {
            buffer: Vec::with_capacity(BLOCK_LEN),
            reader,
            offset: 0,
        }
    }

    #[inline(always)]
    pub fn buffer_from_to(&self, start: usize, end: usize) -> &[u8] {
        &self.buffer[start..end]
    }
    
    pub fn buffer_from(&self, start: usize) -> &[u8] {
        &self.buffer[start..]
    }

    pub fn read_block(&mut self) -> io::Result<bool> {
        let block_len = self.reader.read_u32::<LittleEndian>()?;
        if block_len == 0u32 {
            self.buffer.clear();
            return Ok(false);
        }
        self.buffer.resize(block_len as usize, 0u8);
        self.reader.read_exact(&mut self.buffer[..])?;
        Ok(true)
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn advance(&mut self, num_bytes: usize) {
        self.offset += num_bytes;
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer[self.offset..]
    }
}
