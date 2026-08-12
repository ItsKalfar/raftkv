use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Wal {
    file: File,
}

impl Wal {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new().append(true).create(true).open(path)?;

        Ok(Wal { file: file })
    }

    pub fn write_log(&mut self, message: &str) -> std::io::Result<()> {
        self.file
            .write_all(message.as_bytes())
            .expect("Something went wrong");

        self.file.sync_data()?;

        Ok(())
    }
}
