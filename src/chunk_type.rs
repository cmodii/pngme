#[allow(dead_code)]
#[allow(unused)]

use std::{convert::TryFrom, fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    pub body: [u8; 4]
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidByteRange(u8),
    InvalidString(String)
}

impl std::error::Error for ChunkTypeError {}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkTypeError::InvalidByteRange(byte) => write!(f, "The byte {} is not within ASCII bounds", byte),
            ChunkTypeError::InvalidString(feedback) => write!(f, "Cannot parse chunk type from str because of: {}", feedback)
        }
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.body.clone()
    }

    pub fn is_critical(&self) -> bool {
        if (self.body[0] >> 5) & 1 == 0 {true} else {false}
    }
    
    pub fn is_public(&self) -> bool {
        if (self.body[1] >> 5) & 1 == 0 {true} else {false}
    }
    
    pub fn is_reserved_bit_valid(&self) -> bool {
        if (self.body[2] >> 5) & 1 == 0 {true} else {false}
    }
    
    pub fn is_safe_to_copy(&self) -> bool {
        if (self.body[3] >> 5) & 1 == 1 {true} else {false}
    } 

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && self.body.iter().all(|byte|
            byte.is_ascii_alphabetic()
        )
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;
    
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        bytes.iter().try_for_each(|byte| {
            if !byte.is_ascii_alphabetic() {
                Err(ChunkTypeError::InvalidByteRange(*byte))
            } else {
                Ok(())
            }
        })?;

        Ok(ChunkType { 
            body: bytes,
        })
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_bytes = s.as_bytes();
        if char_bytes.len() != 4 {
            return Err(ChunkTypeError::InvalidString(String::from("Invalid length")));
        }

        char_bytes.iter().try_for_each(|byte| {
            if !byte.is_ascii_alphabetic() {
                Err(ChunkTypeError::InvalidString(String::from("Invalid Character")))
            } else {
                Ok(())
            }
        })?;

        let bytes = char_bytes.try_into().unwrap();
        Ok(ChunkType { 
            body: bytes
         })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = str::from_utf8(&self.body).map_err(|_| fmt::Error)?;
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}