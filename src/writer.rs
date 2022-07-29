use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

use crate::bytestream::{ByteStream, ByteStreamError};
use crate::logiclong::LogicLong;

impl ByteStream {
    pub fn write_byte(&mut self, byte: u8) -> Result<(), ByteStreamError> {
        self.cursor.write_u8(byte)?;
        Ok(())
    }

    pub fn write_bool(&mut self, value: u8, bool: bool) -> Result<(), ByteStreamError> {
        self.write_byte(if bool { value } else { 0 })?;
        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), ByteStreamError> {
        self.cursor.write_all(bytes)?;
        Ok(())
    }

    pub fn write_int8(&mut self, int8: i8) -> Result<(), ByteStreamError> {
        self.cursor.write_i8(int8)?;
        Ok(())
    }

    pub fn write_uint8(&mut self, uint8: u8) -> Result<(), ByteStreamError> {
        self.cursor.write_u8(uint8)?;
        Ok(())
    }

    pub fn write_int16(&mut self, int16: i16) -> Result<(), ByteStreamError> {
        self.cursor.write_i16::<BigEndian>(int16)?;
        Ok(())
    }

    pub fn write_uint16(&mut self, uint16: u16) -> Result<(), ByteStreamError> {
        self.cursor.write_u16::<BigEndian>(uint16)?;
        Ok(())
    }

    pub fn write_int24(&mut self, int24: i32) -> Result<(), ByteStreamError> {
        self.cursor.write_i24::<BigEndian>(int24)?;
        Ok(())
    }

    pub fn write_uint24(&mut self, uint24: u32) -> Result<(), ByteStreamError> {
        self.cursor.write_u24::<BigEndian>(uint24)?;
        Ok(())
    }

    pub fn write_int32(&mut self, int32: i32) -> Result<(), ByteStreamError> {
        self.cursor.write_i32::<BigEndian>(int32)?;
        Ok(())
    }

    pub fn write_int32_le(&mut self, int32: i32) -> Result<(), ByteStreamError> {
        self.cursor.write_i32::<LittleEndian>(int32)?;
        Ok(())
    }

    pub fn write_uint32(&mut self, uint32: u32) -> Result<(), ByteStreamError> {
        self.cursor.write_u32::<BigEndian>(uint32)?;
        Ok(())
    }

    pub fn write_int64(&mut self, int64: i64) -> Result<(), ByteStreamError> {
        self.cursor.write_i64::<BigEndian>(int64)?;
        Ok(())
    }

    pub fn write_uint64(&mut self, uint64: u64) -> Result<(), ByteStreamError> {
        self.cursor.write_u64::<BigEndian>(uint64)?;
        Ok(())
    }

    pub fn write_vint(&mut self, mut vint: i64) -> Result<(), ByteStreamError> {
        let mut tmp = (vint >> 25) & 0x40;
        let mut flipped = vint ^ (vint >> 31);

        tmp |= vint & 0x3F;
        vint >>= 6;

        flipped >>= 6;
        if flipped == 0 {
            self.write_byte(tmp as u8)?;
            return Ok(());
        }

        self.write_byte((tmp | 0x80) as u8)?;
        loop {
            flipped >>= 7;
            let or_value = if flipped != 0 { 0x80 } else { 0 };
            self.write_byte(((vint & 0x7F) | or_value) as u8)?;
            vint >>= 7;
            if flipped == 0 {
                break;
            }
        }
        Ok(())
    }

    pub fn write_long(&mut self, long: i64) -> Result<(), ByteStreamError> {
        self.cursor.write_i64::<BigEndian>(long)?;
        Ok(())
    }

    pub fn write_ulong(&mut self, ulong: u64) -> Result<(), ByteStreamError> {
        self.cursor.write_u64::<BigEndian>(ulong)?;
        Ok(())
    }

    pub fn write_longlong(&mut self, longlong: i64) -> Result<(), ByteStreamError> {
        self.cursor.write_i64::<BigEndian>(longlong)?;
        Ok(())
    }

    pub fn write_ulonglong(&mut self, ulonglong: u64) -> Result<(), ByteStreamError> {
        self.cursor.write_u64::<BigEndian>(ulonglong)?;
        Ok(())
    }

    pub fn write_string(&mut self, string: String) -> Result<(), ByteStreamError> {
        // write length of string as i32, then string
        let length = string.len() as i32;
        if length == 0 || length == -1 {
            self.cursor.write_i32::<BigEndian>(-1)?;
            return Ok(());
        }
        self.cursor.write_i32::<BigEndian>(length)?;
        self.cursor.write_all(string.as_bytes())?;
        Ok(())
    }

    pub fn write_string_reference(&mut self, string: String) -> Result<(), ByteStreamError> {
        // write length of string as i32, then string
        let length = string.len() as i32;
        if length == 0 || length == -1 {
            self.cursor.write_i32::<BigEndian>(0)?; // difference here -
            return Ok(());
        }
        self.cursor.write_i32::<BigEndian>(length)?;
        self.cursor.write_all(string.as_bytes())?;
        Ok(())
    }

    pub fn write_string_size(
        &mut self,
        size: usize,
        string: String,
    ) -> Result<(), ByteStreamError> {
        // write size as i32, then string for size bytes
        self.cursor.write_i32::<BigEndian>(size as i32)?;
        // only write size bytes of string
        self.cursor.write_all(&string.as_bytes()[..size])?;
        Ok(())
    }

    pub fn write_string_size_reference(
        &mut self,
        size: usize,
        string: String,
    ) -> Result<(), ByteStreamError> {
        // write size as i32, then string for size bytes
        self.cursor.write_i32::<BigEndian>(size as i32)?;
        // only write size bytes of string
        self.cursor.write_all(&string.as_bytes()[..size])?;
        Ok(())
    }

    pub fn write_compressed_string(&mut self, string: String) -> Result<(), ByteStreamError> {
        // write length of compressed data as i32, then uncompressed data as i32 LE, then compressed data bytes
        let uncompressed_size = string.len() as i32; // write LE

        let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
        compressor.write_all(string.as_bytes())?;
        let compressed_data = compressor.finish()?;
        let compressed_size = compressed_data.len() as i32;

        self.cursor.write_i32::<BigEndian>(compressed_size)?;
        self.cursor.write_i32::<LittleEndian>(uncompressed_size)?;
        self.cursor.write_all(&compressed_data)?;

        Ok(())
    }

    pub fn write_logic_long(&mut self, logic_long: LogicLong) -> Result<(), ByteStreamError> {
        self.cursor.write_u32::<BigEndian>(logic_long.low)?;
        self.cursor.write_u32::<BigEndian>(logic_long.high)?;
        Ok(())
    }
}
