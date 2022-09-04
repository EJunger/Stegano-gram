use crate::png::chunk_type::ChunkType;

use std::error::Error;
use std::fmt::Display;
use std::convert::TryFrom;

use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    pub chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;

        let crc_algo: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let chunk_type_bytes = chunk_type.bytes();
        let crc_bytes = [&chunk_type_bytes, data.as_slice()].concat();
        let crc: u32 = crc_algo.checksum(&crc_bytes);

        Self {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, Box<dyn Error>> {
        let owned_vec = self.data.to_owned();
        Ok(String::from_utf8(owned_vec).unwrap())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length = self.length;
        let chunk_type = self.chunk_type.bytes();
        let data = self.data.clone();
        let crc = self.crc;

        let chunk_data: Vec<u8> = length.to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(data.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        chunk_data
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_bytes = self.chunk_type.bytes();
        write!(
            f,
            "{:?}{:?}{:?}{:?}",
            self.length, type_bytes, self.data, self.crc
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let crc_algo: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc_offset = bytes.len() - 4;

        let length = &bytes[0..4];
        let chunk_type = &bytes[4..8];
        let crc_raw = &bytes[crc_offset..];
        let data_raw = &bytes[8..crc_offset];

        let crc = u32::from_be_bytes(crc_raw.try_into().unwrap());
        let data = data_raw.to_vec();

        let crc_test_data = [chunk_type, data_raw].concat();
        let checksum = crc_algo.checksum(&crc_test_data);

        if crc != checksum {
            return Err("!!Invalid Chunk!!")
        }
        
        Ok(Chunk {
            length: u32::from_be_bytes(length.try_into().unwrap()),
            chunk_type: ChunkType::try_from([
                chunk_type[0],
                chunk_type[1],
                chunk_type[2],
                chunk_type[3],
            ]).unwrap(),
            data,
            crc,        
        })
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::*;
    use crate::png::chunk_type::ChunkType;
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
