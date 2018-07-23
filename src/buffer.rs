// A buffer is a text document
// You will be able to edit multiple buffers at a time

use std::io::{BufReader, Result};
use std::fs::File;
use ropey::Rope;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub text: Rope,
    pub name: Option<String>,
    pub position: usize,
}

impl Buffer {
    pub fn new(filename: Option<&str>) -> Result<Buffer> {
        if let Some(filename) = filename {
            Ok(Buffer {
                text: Rope::from_reader(BufReader::new(File::open(filename)?))?,
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

    pub fn name_buffer(&mut self, filename: &str) -> Option<Buffer> {
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
}
