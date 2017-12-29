use super::syntax::{
    Location
};
use std::{
    fs,
    io,
};
use std::io::{
    Read,
};
use utils::{
    HeadedList,
};




#[derive(Clone, Debug)]
pub struct ByteStream {
    bytes: Vec<u8>,
    pos: usize,
    file: String,
    line: u64,
    cols: HeadedList<u64>,
}

impl ByteStream {

    pub fn from_str(file: String, bytes: &str) -> Self {
        Self::from_bstr(file, bytes.bytes().collect())
    }

    pub fn from_bstr(file: String, bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            pos: 0,
            file,
            line: 1,
            cols: HeadedList::new(1, None),
        }
    }

    pub fn from_file(file: String) -> Result<Self, io::Error> {
        let mut stream = match fs::File::open(&file) {
            Err(e) => return Err(e),
            Ok(s) => s,
        };
        let mut bytes = Vec::new();
        match stream.read_to_end(&mut bytes) {
            Err(e) => Err(e),
            _ => Ok(Self::from_bstr(file, bytes)),
        }
    }

    pub fn loc(&self) -> Location {
        Location {
            file: self.file.clone(),
            line: self.line,
            column: self.cols.val_cpy(),
        }
    }

    pub fn current(&self) -> Option<u8> {
        match self.bytes.get(self.pos) {
            Some(&x) => Some(x),
            _ => None,
        }
    }

    pub fn next(&mut self) -> bool {
        match self.current() {
            Some(ch) => {
                if ch == b'\n' {
                    self.line += 1;
                    self.cols.receive(1);
                } else {
                    *self.cols.val_mut() += 1;
                }
                self.pos += 1;
                true
            }
            _ => false,
        }
    }

    pub fn previous(&mut self) -> bool {
        match self.current() {
            Some(ch) => if self.pos == 0 {false} else {
                if ch == b'\n' {
                    self.line -= 1;
                    self.cols.take();
                } else {
                    *self.cols.val_mut() -= 1;
                }
                self.pos -= 1;
                true
            },
            _ => false,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    pub fn is_start(&self) -> bool {
        self.pos == 0
    }

}


