use std::{str::FromStr, num::ParseIntError, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType(pub u32); 

#[derive(Debug)]
pub enum ChunkError {
    Parsing(ParseIntError)
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        u32::to_be_bytes(self.0)
    }
    
    pub fn is_valid(&self) -> bool {
        let bytes = u32::to_be_bytes(self.0).into_iter();
        for (i, byte) in bytes.enumerate() {
            match byte {
                _ if i == 2 && !byte.is_ascii_uppercase() => return false,
                _ if !byte.is_ascii_lowercase() && 
                    !byte.is_ascii_uppercase() => return false,
                _ => {}
            }
        }
        true
    }
    pub fn is_err(&self) -> bool {
        let bytes = u32::to_be_bytes(self.0).into_iter();
        for (i, byte) in bytes.enumerate() {
            match byte {
                _ if i == 2 && !byte.is_ascii_uppercase() => return true,
                _ if !byte.is_ascii_lowercase() && 
                    !byte.is_ascii_uppercase() => return true,
                _ => {}
            }
        }
        false
    }

    fn set_bitness(&self, bit_position: usize) -> bool {
        for (i, byte) in self.bytes().iter().enumerate() {
            if i == bit_position && byte.is_ascii_uppercase() {
                return true;
            }
        }
        false
    }

    pub fn is_critical(&self) -> bool {
       self.is_valid() && self.set_bitness(0)
    }

    pub fn is_public(&self) -> bool {
        self.is_valid() && self.set_bitness(1)
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.is_valid() && self.set_bitness(2)
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.is_valid() && !self.set_bitness(3)
    }
    
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType(u32::from_be_bytes(value)))
    }
}

impl FromStr for ChunkType {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: [u8; 4] = [0u8; 4];
        for (i, char) in s.chars().enumerate() {
            chars[i] = char as u8;
        }
        Ok(ChunkType(u32::from_be_bytes(chars)))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.0.to_be_bytes();
        let chunk_str = match std::str::from_utf8(&bytes) {
            Ok(val) => val,
            Err(err) => panic!("invalid utf8 {}", err) 
        };
        write!(f, "{}", chunk_str)
    }
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)       
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

        let chunk = ChunkType::from_str("Ru1t").unwrap();
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

