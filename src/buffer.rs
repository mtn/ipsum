// A buffer is a text document
// You will be able to edit multiple buffers at a time

use ropey::Rope;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Result};

#[derive(Debug, Clone)]
pub struct Buffer {
    pub text: Rope,
    pub name: Option<String>,
    pub position: usize,
}

impl Buffer {
    pub fn new(filename: Option<&str>) -> Result<Buffer> {
        if let Some(filename) = filename {
            let file = Self::open_file(filename);

            Ok(Buffer {
                text: Rope::from_reader(BufReader::new(file?))?,
                name: Some(String::from(filename)),
                position: 0,
            })
        } else {
            Ok(Buffer {
                text: Rope::new(),
                name: None,
                position: 0,
            })
        }
    }

    fn open_file(filename: &str) -> Result<File> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
    }

    pub fn set_name(&mut self, filename: &str) -> Option<Buffer> {
        // If the buffer isn't named, it is named in its first write to disk
        if self.name == None {
            self.name = Some(String::from(filename));

            return None;
        }

        // If it is named, we clone it, and the two buffers are allowed to diverge
        // The editor will keep track of the multiple buffers
        let mut new_named_buffer = self.clone();
        new_named_buffer.name = Some(String::from(filename));

        Some(new_named_buffer)
    }

    /// Write the buffer to disk
    // Todo return messages on failures, do be displayed in the status bar
    pub fn save(&mut self) -> bool {
        if self.name == None {
            return false;
        }

        // let file = Self::open_file(&self.name.unwrap());
        unimplemented!();
    }
}
