use std::fmt::Display;
use crate::{Result, Error};
use crate::chunk_type::ChunkType;
use crc32fast;

#[derive(Debug, Clone)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>
}

impl Chunk {
    
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {

        Chunk {
            chunk_type, 
            data
        }

    }

    /// A 4-byte unsigned integer giving the number of bytes in the chunk's data field. 
    /// The length counts only the data field, not itself, the chunk type code, or the CRC. 
    /// Zero is a valid length. 
    /// Although encoders and decoders should treat the length as unsigned, 
    /// its value must not exceed 231 bytes.
    pub fn length(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }

    /// A 4-byte chunk type code. 
    /// For convenience in description and in examining PNG files, 
    /// type codes are restricted to consist of uppercase and lowercase ASCII letters 
    /// (A-Z and a-z, or 65-90 and 97-122 decimal). 
    /// However, encoders and decoders must treat the codes as fixed binary values, 
    /// not character strings. 
    /// For example, it would not be correct to represent the type code IDAT 
    /// by the EBCDIC equivalents of those letters. 
    /// Additional naming conventions for chunk types are discussed in the next section.
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The data bytes appropriate to the chunk type, if any. 
    /// This field can be of zero length.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// A 4-byte CRC (Cyclic Redundancy Check) calculated on the preceding bytes in the chunk, 
    /// including the chunk type code and chunk data fields, but not including the length field. 
    /// The CRC is always present, even for chunks containing no data. 
    /// See [CRC algorithm](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#CRC-algorithm).
    pub fn crc(&self) -> u32 {

        // bytes consisting of chunk type and chunk data
        let bytes: Vec<u8> = self.chunk_type.bytes().iter()
            .chain(self.data.iter())
            .copied()
            .collect();

        // compute CRC
        crc32fast::hash(&bytes)

    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = String::from_utf8(self.data.clone());
        match s {
            Ok(s) => Ok(s),
            Err(_) => Err(Box::new(ChunkError::StringConvertionFailure))
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {

        self.length().to_be_bytes().iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()

    }

}

impl TryFrom<&[u8]> for Chunk {

    type Error = Error;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {

        // a vector of input bytes
        let mut bytes = value.to_vec();

        // bytes representing the data length
        let data_length_bytes: [u8; 4] = bytes.drain(0..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        // convert to length
        let data_length = u32::from_be_bytes(data_length_bytes);
        let data_length: usize = data_length.try_into().unwrap();

        // bytes representing chunk type
        let chunk_type_bytes: [u8; 4] = bytes.drain(0..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        // convert to chunk type
        let chunk_type = ChunkType::try_from(chunk_type_bytes).unwrap();

        // message data bytes
        let data: Vec<u8> = bytes.drain(0..data_length).collect();

        // bytes representing the CRC value
        let crc_bytes: [u8; 4] = bytes.drain(0..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        // recover the CRC value
        let crc = u32::from_be_bytes(crc_bytes);

        // create the chunk object
        let chunk = Chunk::new(chunk_type, data);

        // check CRC
        if chunk.crc() == crc {
            Ok(chunk)
        } else {
            Err(Box::new(ChunkError::CRCMismatch))
        }

    }
    
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{}", self.data_as_string().unwrap())

    }
}

#[derive(Debug)]
pub enum ChunkError {
    StringConvertionFailure,
    CRCMismatch,
    InvalidNumberOfBytes
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringConvertionFailure => {
                write!(f, "{}", "failed in converting to string")
            },
            Self::CRCMismatch => {
                write!(f, "{}", "the CRC value extracted from the input bytes does not match that of the message data")
            },
            Self::InvalidNumberOfBytes => {
                write!(f, "{}", "the number of input bytes is invalid")
            }
        }
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
