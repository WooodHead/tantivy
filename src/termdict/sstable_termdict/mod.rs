use std::io;

use tantivy_sstable::{BlockReader, SSTable};
use tantivy_sstable::value::{ValueReader, ValueWriter};

use crate::postings::TermInfo;

mod term_dict;

pub struct TermSSTable;

impl SSTable for TermSSTable {
    type Value = TermInfo;
    type Reader = TermInfoReader;
    type Writer = TermInfoWriter;
}

#[derive(Default)]
pub struct TermInfoReader {
    term_info: TermInfo
}

impl ValueReader for TermInfoReader {
    type Value = TermInfo;

    fn value(&self) -> &TermInfo {
        &self.term_info
    }

    fn read(&mut self, reader: &mut BlockReader) -> io::Result<()> {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct TermInfoWriter {

}

impl ValueWriter for TermInfoWriter {
    type Value = TermInfo;

    fn write(&mut self, val: &TermInfo, writer: &mut Vec<u8>) {
        unimplemented!()
    }
}