use bcs_core::global::bcsstring::BcsStringWrapper;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// BcsTextStream provides a convenient interface for reading and writing text.
pub struct BcsTextStream {
    reader: Option<BufReader<File>>,
    writer: Option<File>,
}

impl BcsTextStream {
    pub fn new(file: File) -> Self {
        // Conceptually maps the stream to underlying file handles
        let file_clone = file.try_clone().unwrap(); // Simplification for port mapping
        BcsTextStream {
            reader: Some(BufReader::new(file)),
            writer: Some(file_clone),
        }
    }

    pub fn read_line(&mut self) -> io::Result<BcsStringWrapper> {
        let mut buffer = String::new();
        if let Some(ref mut reader) = self.reader {
            reader.read_line(&mut buffer)?;
        }
        Ok(BcsStringWrapper::new(&buffer))
    }

    pub fn write_string(&mut self, str: &BcsStringWrapper) -> io::Result<()> {
        if let Some(ref mut writer) = self.writer {
            writer.write_all(str.to_utf8().as_slice())?;
            writer.flush()?;
        }
        Ok(())
    }
}
