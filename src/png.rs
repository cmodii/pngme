use crate::{chunk::Chunk, chunk_type::ChunkType};
use std::io::{Cursor, Read, SeekFrom, Seek};

pub struct Png {
    header: [u8;8],
    chunks: Vec<Chunk>
}

pub enum PNGError {
    InsufficientBits,
    HeaderMismatch,
    ReadErr(std::io::Error),
    ChunkParse(String),
}

impl Png {
    pub const STANDARD_HEADER: [u8;8] = [137, 80, 78, 71, 13, 10, 26, 10]; 
}


impl TryFrom<&[u8]> for Png {
    type Error = self::PNGError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(PNGError::InsufficientBits);
        }

        let mut cursor = Cursor::new(bytes);
        let mut chunks: Vec<Chunk> = Vec::new();

        let mut header_buf: [u8;8] = [0;8];
        let mut length_buf: [u8;4] = [0;4];

        cursor.read_exact(&mut header_buf).map_err(|e| PNGError::ReadErr(e))?;
        if header_buf != Png::STANDARD_HEADER {
            return Err(PNGError::HeaderMismatch);
        }
        
        loop {
            cursor.read_exact(&mut length_buf).map_err(|e| PNGError::ReadErr(e))?;
            cursor.seek(SeekFrom::Current(-(length_buf.len() as i64))).map_err(|e| PNGError::ReadErr(e))?;

            let length: u32 = u32::from_be_bytes(length_buf);
            let mut chunk_data: Vec<u8> = vec![0; (12 + length) as usize];
            cursor.read_exact(&mut chunk_data).map_err(|e| PNGError::ReadErr(e))?;

            let chunk = Chunk::try_from(&chunk_data[0..chunk_data.len()]).map_err(|s| PNGError::ChunkParse(s.to_string()))?;
            chunks.push(chunk);
            // if bytes empty then break
            todo!()
        }
    }
}