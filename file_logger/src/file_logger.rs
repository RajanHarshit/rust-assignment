use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
/// File-based logger
pub struct FileLogger {
    pub file: File,
}

impl FileLogger {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;

        Ok(FileLogger { file })
    }
}