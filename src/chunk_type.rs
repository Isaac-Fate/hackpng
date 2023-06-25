use std::{
    str::FromStr, 
    fmt::Display
};
use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType {
    bytes: [u8; 4]
}

impl ChunkType {

    /// Return the 4 bytes representing the chunk type.
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Check if the chunk type code is valid.
    pub fn is_valid(&self) -> bool {
        self.bytes.iter().all(|byte| byte.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }
    
    pub fn is_critical(&self) -> bool {
        Self::is_bit5_zero(self.bytes[0])
    }

    pub fn is_public(&self) -> bool {
        Self::is_bit5_zero(self.bytes[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::is_bit5_zero(self.bytes[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !Self::is_bit5_zero(self.bytes[3])
    }

    /// Check whether the 5-th bit (value 32) of the given byte is zero.
    /// In fact, it is equivalent to the function `is_ascii_uppercase()`
    /// belonging to `u8`.
    fn is_bit5_zero(byte: u8) -> bool {
        byte & (1 << 5) == 0
    }

}


impl TryFrom<[u8; 4]> for ChunkType {

    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {

        let chunk = ChunkType {
            bytes: value
        };
        
        Ok(chunk)

    }

}

impl FromStr for ChunkType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let bytes = s.as_bytes();

        if bytes.len() != 4 {
            return Err(Box::new(ChunkTypeError::UnexpectedLength(bytes.len())));
        } 
        
        let bytes: [u8; 4] = (&bytes[0..4]).try_into().or_else(|_| {
            return Err(Box::new(ChunkTypeError::UnexpectedLength(bytes.len())));
        })?;

        if !bytes.iter().all(|byte| byte.is_ascii_alphabetic()) {
            return Err(Box::new(ChunkTypeError::InvalidCharacter));
        }

        Ok(ChunkType { bytes })

    }

}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.bytes).or_else(|_| Err(std::fmt::Error))?;
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    /// The expected length of input string is 4.
    UnexpectedLength(usize),

    /// Every character in the input string must be an ASCII letter.
    InvalidCharacter
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::UnexpectedLength(n_bytes) => {
                write!(f, "the expected length is 4 while the given string has length {}", n_bytes)
            },

            ChunkTypeError::InvalidCharacter => {
                write!(f, "every character must be an ASCII letter")
            }
        }
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
