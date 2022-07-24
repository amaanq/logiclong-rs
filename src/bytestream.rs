use std;
use std::io::Cursor;

#[derive(Debug)]
pub struct ByteStream {
    pub cursor: Cursor<Vec<u8>>,
    pub offset: usize,
    pub length: usize,
    // reader: std::io::BufReader,
    // writer: std::io::BufWriter,
}

impl ByteStream {
    // meant for reading
    pub fn new_from_buffer(buffer: Vec<u8>) -> ByteStream {
        ByteStream {
            length: buffer.len(),
            offset: 0,
            cursor: Cursor::new(buffer),
        }
    }

    // meant for writing
    pub fn new() -> ByteStream {
        let mut buffer = Vec::new();
        ByteStream {
            length: 0,
            offset: 0,
            cursor: Cursor::new(buffer),
        }
    }
}

#[derive(Debug)]
pub enum ByteStreamError {
    IoError(std::io::Error),
    InvalidBytesRead(usize),
    InvalidBytesWritten(usize),
    InvalidStringLength(usize),
    InvalidString(String),
    NotEnoughBytes,
    NoMoreBytes,
}

impl From<std::io::Error> for ByteStreamError {
    fn from(error: std::io::Error) -> Self {
        ByteStreamError::IoError(error)
    }
}
