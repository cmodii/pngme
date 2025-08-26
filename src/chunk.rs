#[allow(dead_code)]
#[allow(unused)]
use std::{io::BufReader, io::Read, string::FromUtf8Error};
use crate::{chunk_type::ChunkType};
use core::fmt;

pub const CRC32_LOOKUP_TABLE: [u32; 256] = {
    let mut table: [u32; 256] = [0;256];

    let mut crc: u32;
    let mut byte = 0; let mut bit = 0;
    while byte < 256 {
        crc = byte;
        while bit < 8 {
            if (crc & 1) != 0 {
                crc = 0xEDB88320 ^ (crc >> 1);
            } else {
                crc >>= 1;
            }
            bit+=1;
        }
        table[byte as usize] = crc;
        bit = 0;
        byte+=1;
    }

    table
};

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug)]
pub enum ChunkError {
    ReadErr(std::io::Error),
    DataLength(usize),
    CRCMismatch(u32, u32)
}

impl std::error::Error for ChunkError {}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkError::ReadErr(err) => write!(f, "Byte reading failed. Returned: {}", err),
            ChunkError::DataLength(size) => write!(f, "Data size insufficient to parse into a valid chunk ({} < 12)", size),
            ChunkError::CRCMismatch(correct, incorrect) => write!(f, "Chunk contains {:#x} as CRC value when it should contain {:#x}", incorrect, correct)
        }
    }
}

pub fn crc32(input_str: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;

    for &byte in input_str {
        let lookup_index = ((crc ^ u32::from(byte)) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_LOOKUP_TABLE[lookup_index];
    }
    crc ^ 0xFFFFFFFF
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let stream: Vec<u8> = chunk_type.bytes()
                .into_iter()
                .chain(data.clone())
                .collect();
        let crc = crc32(&stream);

        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            data: data,
            crc: crc   
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        let string = String::from_utf8(self.data.clone())?;
        Ok(string)
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .into_iter()
            .chain(self.chunk_type.bytes())
            .chain(self.data.clone())
            .chain(self.crc.to_be_bytes())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let length = data.len();
        if length < 12 {
            return Err(ChunkError::DataLength(data.len()));
        }

        let mut reader = BufReader::new(data);
        let mut length_buf: [u8; 4] = [0;4];
        let mut chunk_buf: [u8; 4] = [0;4];
        let mut crc_buf: [u8;4] = [0;4];
        
        reader.read_exact(&mut length_buf).map_err(|err| ChunkError::ReadErr(err))?;
        let length = u32::from_be_bytes(length_buf);
        
        reader.read_exact(&mut chunk_buf).map_err(|err| ChunkError::ReadErr(err))?;
        let chunk_type = ChunkType::try_from(chunk_buf).unwrap();
        
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data).map_err(|err| ChunkError::ReadErr(err))?;

        reader.read_exact(&mut crc_buf).map_err(|err| ChunkError::ReadErr(err))?;
        let crc = u32::from_be_bytes(crc_buf);
        let crc_stream: Vec<u8> = chunk_buf
                .into_iter()
                .chain(data.clone())
                .collect();

        if crc != crc32(&crc_stream) {
            return Err(ChunkError::CRCMismatch(crc, crc32(&crc_stream)));
        }

        Ok(Chunk {
            length: length,
            chunk_type: chunk_type,
            data: data,
            crc: crc
        })
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n")?;
        write!(f, " [Data Length]: {}\n", self.length)?;
        write!(f, " [Chunk Type]: {}\n", String::from_utf8(self.chunk_type.body.to_vec()).unwrap())?;
        if self.data_as_string().is_ok() {
            write!(f, " [Data]: {}\n", self.data_as_string().unwrap())?;
        } else {
            write!(f, " [Data]: INVALID_UTF8_STRING\n")?;
        }
        write!(f, " [CRC32-ISO-HDLC]: {}\n", self.crc)?;
        write!(f, "}}\n")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}