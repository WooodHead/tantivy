use std::io;

use crate::postings::TermInfo;
use crate::termdict::TermOrdinal;
use crate::termdict::sstable_termdict::{TermInfoWriter, TermSSTable};
use tantivy_sstable::SSTable;
use tantivy_sstable::Writer;


pub struct TermDictionaryBuilder<W: io::Write> {
    sstable_writer: Writer<W, TermInfoWriter>,
    term_ord: usize,
}

impl<W: io::Write> TermDictionaryBuilder<W> {
   /// Creates a new `TermDictionaryBuilder`
    pub fn create(w: W) -> io::Result<Self> {
        let sstable_writer = TermSSTable::writer(w);
        Ok(TermDictionaryBuilder {
            sstable_writer,
            term_ord: 0
        })
    }

    /// Inserts a `(key, value)` pair in the term dictionary.
    ///
    /// *Keys have to be inserted in order.*
    pub fn insert<K: AsRef<[u8]>>(&mut self, key_ref: K, value: &TermInfo) -> io::Result<()> {
        let key = key_ref.as_ref();
        self.insert_key(key)?;
        self.insert_value(value)?;
        Ok(())
    }

    /// # Warning
    /// Horribly dangerous internal API
    ///
    /// If used, it must be used by systematically alternating calls
    /// to insert_key and insert_value.
    ///
    /// Prefer using `.insert(key, value)`
    pub(crate) fn insert_key(&mut self, key: &[u8]) -> io::Result<()> {
        self.sstable_writer.write_key(key);
        self.term_ord += 1;
        Ok(())
    }

    /// # Warning
    ///
    /// Horribly dangerous internal API. See `.insert_key(...)`.
    pub(crate) fn insert_value(&mut self, term_info: &TermInfo) -> io::Result<()> {
        self.sstable_writer.write_value(term_info);
        Ok(())
    }

    /// Finalize writing the builder, and returns the underlying
    /// `Write` object.
    pub fn finish(mut self) -> io::Result<W> {
        self.sstable_writer.finalize()
    } 
}