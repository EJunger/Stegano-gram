use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

// TODO:
// double check resources page for better conversion methods

// ancillary_bit    - 0 (uppercase) = critical, 1 (lowercase) = ancillary.
// private_bit      - 0 (uppercase) = public, 1 (lowercase) = private.
// reserved_bit     - Must be 0 (uppercase) in files conforming to this version of PNG.
// safe_to_copy_bit - 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    pub ancillary_bit: char,
    pub private_bit: char,
    pub reserved_bit: char,
    pub safe_to_copy_bit: char,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary_bit as u8,
            self.private_bit as u8,
            self.reserved_bit as u8,
            self.safe_to_copy_bit as u8,
        ]
    }

    //NOTE: something feels off, could more be done? or diff? w/e it workky
    fn is_valid(&self) -> bool {
        self.reserved_bit.is_ascii_uppercase()
    }

    fn is_critical(&self) -> bool {
        self.ancillary_bit.is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.private_bit.is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.reserved_bit.is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy_bit.is_ascii_lowercase()
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.ancillary_bit, self.private_bit, self.reserved_bit, self.safe_to_copy_bit
        )
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|x| x.is_numeric()) {
            return Err("!!Invalid chunk type!!");
        }
        let char_vec: Vec<char> = s.chars().collect();

        Ok(Self {
            ancillary_bit: char_vec[0],
            private_bit: char_vec[1],
            reserved_bit: char_vec[2],
            safe_to_copy_bit: char_vec[3],
        })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Box<dyn Error>; //NOTE: ParseIntError instead maybe

    fn try_from(bits: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self {
            ancillary_bit: bits[0] as char,
            private_bit: bits[1] as char,
            reserved_bit: bits[2] as char,
            safe_to_copy_bit: bits[3] as char,
        })
    }
}

#[cfg(test)]
mod type_tests {
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
