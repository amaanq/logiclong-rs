use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use std::io::Read;

use crate::bytestream::{ByteStream, ByteStreamError};
use crate::logiclong::LogicLong;

impl ByteStream {
    // read a byte from the buffer
    pub fn read_byte(&mut self) -> Result<u8, ByteStreamError> {
        let byte = self.cursor.read_u8()?;
        Ok(byte)
    }

    // read bool will read a byte and return true and the value of the byte if it is > 0, or false and 0 if it is 0
    pub fn read_bool(&mut self) -> Result<(bool, u8), ByteStreamError> {
        let byte = self.read_byte()?;
        let bool = byte > 0;
        Ok((bool, byte))
    }

    // read n bytes from the buffer if possible, and return a reference to the bytes
    pub fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ByteStreamError> {
        // let mut bytes = Vec::new();
        (0..n).map(|_| self.read_byte()).collect()
    }

    // read int8 will read a byte and return the value of the byte
    pub fn read_int8(&mut self) -> Result<i8, ByteStreamError> {
        let int8 = self.cursor.read_i8()?;
        Ok(int8)
    }

    // read uint8 will read a byte and return the value of the byte
    pub fn read_uint8(&mut self) -> Result<u8, ByteStreamError> {
        let uint8 = self.cursor.read_u8()?;
        Ok(uint8)
    }

    // read int16 will read 2 bytes and return the value of the bytes
    pub fn read_int16(&mut self) -> Result<i16, ByteStreamError> {
        let int16 = self.cursor.read_i16::<BigEndian>()?;
        Ok(int16)
    }

    // read uint16 will read 2 bytes and return the value of the bytes
    pub fn read_uint16(&mut self) -> Result<u16, ByteStreamError> {
        let uint16 = self.cursor.read_u16::<BigEndian>()?;
        Ok(uint16)
    }

    // read int24 will read 3 bytes and return the value of the bytes as an i32
    pub fn read_int24(&mut self) -> Result<i32, ByteStreamError> {
        let int24 = self.cursor.read_i24::<BigEndian>()?;
        Ok(int24)
    }

    // read uint24 will read 3 bytes and return the value of the bytes as a u32
    pub fn read_uint24(&mut self) -> Result<u32, ByteStreamError> {
        let uint24 = self.cursor.read_u24::<BigEndian>()?;
        Ok(uint24)
    }

    // read int32 will read 4 bytes and return the value of the bytes as an i32
    pub fn read_int32(&mut self) -> Result<i32, ByteStreamError> {
        let int32 = self.cursor.read_i32::<BigEndian>()?;
        Ok(int32)
    }

    // read int32LE will read 4 bytes and return the value of the bytes as an i32 in little endian
    pub fn read_int32LE(&mut self) -> Result<i32, ByteStreamError> {
        let int32 = self.cursor.read_i32::<LittleEndian>()?;
        Ok(int32)
    }

    // read uint32 will read 4 bytes and return the value of the bytes as a u32
    pub fn read_uint32(&mut self) -> Result<u32, ByteStreamError> {
        let uint32 = self.cursor.read_u32::<BigEndian>()?;
        Ok(uint32)
    }

    // read int64 will read a byte and return the value of the byte as an i64
    pub fn read_int64(&mut self) -> Result<i64, ByteStreamError> {
        let int64 = self.cursor.read_i64::<BigEndian>()?;
        Ok(int64)
    }

    // read uint64 will read 8 bytes and return the value of the bytes as a u64
    pub fn read_uint64(&mut self) -> Result<u64, ByteStreamError> {
        let uint64 = self.cursor.read_u64::<BigEndian>()?;
        Ok(uint64)
    }

    // read varint will read as many bytes as necessary to read a varint and return the value of the varint as an i64
    pub fn read_vint(&mut self) -> Result<i64, ByteStreamError> {
        let b = self.read_byte()? as i64;
        let sign = (b >> 6) & 1;
        let mut i = b & 0x3F;
        let mut offset = 6 as i64;

        for _ in 0..4 {
            if (b & 0x80) != 0 {
                let b = self.read_byte()? as i64;
                i |= (b & 0x7F) << offset;
                offset += 7;
            } else {
                break;
            }
        }

        Ok(if (b & 0x80) != 0 {
            -1
        } else {
            if sign == 1 && offset < 32 {
                i | (i | (0xFFFFFFFF << offset))
            } else {
                i
            }
        })
    }

    // read long = read int64
    pub fn read_long(&mut self) -> Result<i64, ByteStreamError> {
        let long = self.cursor.read_i64::<BigEndian>()?;
        Ok(long)
    }

    // read ulong = read uint64
    pub fn read_ulong(&mut self) -> Result<u64, ByteStreamError> {
        let ulong = self.cursor.read_u64::<BigEndian>()?;
        Ok(ulong)
    }

    // read longlong = read int64
    pub fn read_longlong(&mut self) -> Result<i64, ByteStreamError> {
        let longlong = self.cursor.read_i64::<BigEndian>()?;
        Ok(longlong)
    }

    // read ulonglong = read uint64
    pub fn read_ulonglong(&mut self) -> Result<u64, ByteStreamError> {
        let ulonglong = self.cursor.read_u64::<BigEndian>()?;
        Ok(ulonglong)
    }

    // read string will read a 4 byte i32 (n) declaring the length of the string, and then read n bytes from the buffer as a string
    pub fn read_string(&mut self) -> Result<String, ByteStreamError> {
        // read int32, then read that many bytes as a string
        let length = self.read_int32()?;
        if length < -1 {
            return Err(ByteStreamError::InvalidStringLength(length as usize));
        } else if length == 0 || length == -1 {
            return Ok(String::new());
        }

        self.read_string_size(length as usize)
    }

    // read string_reference = read string
    pub fn read_string_reference(&mut self) -> Result<String, ByteStreamError> {
        // read int32, then read that many bytes, same for reading but different when writing
        self.read_string()
    }

    // read string size will read a string given the size already
    pub fn read_string_size(&mut self, size: usize) -> Result<String, ByteStreamError> {
        // read size bytes as a string
        let mut bytes = vec![0; size];
        self.cursor.read_exact(&mut bytes)?;
        Ok(String::from_utf8(bytes).map_err(|e| ByteStreamError::InvalidString(e.to_string()))?)
    }

    // read compressed_string reads a 4 byte compressed length, 4 byte LE uncompressed length,
    // reads the compressed string, and then decompresses it
    pub fn read_compressed_string(&mut self) -> Result<String, ByteStreamError> {
        let compressed_size = self.read_int32()?;
        let _uncompressed_size = self.read_int32LE()?;
        let compressed_bytes = self.read_bytes(compressed_size as usize)?;

        let mut decompressor = ZlibDecoder::new(&compressed_bytes[..]);
        let mut data = String::new();
        decompressor.read_to_string(&mut data)?;
        Ok(data)
    }

    // custom 2 4 byte ints that represent a game player tag, see logiclong.rs for more info
    pub fn read_logic_long(&mut self) -> Result<LogicLong, ByteStreamError> {
        let (low, high) = (self.read_int32()?, self.read_int32()?);
        Ok(LogicLong::new(low, high))
    }
}
