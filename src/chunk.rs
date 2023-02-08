use std::{fmt::{Display, Debug}, io};

use crc::{Crc, Algorithm};

use crate::chunk_type::ChunkType;

const PNG_CRC_ALGO: Algorithm<u32> = Algorithm { 
            width: 32, 
            poly: 0x04C11DB7, 
            init: 0xFFFFFFFF, 
            refin: true, 
            refout: true, 
            xorout: 0xFFFFFFFF, 
            check: 0xCBF43926, 
            residue: 0x00000000 
};

#[derive(Debug)]
pub struct Chunk {
    pub data_length: u32,
    pub chunk_type: ChunkType,
    pub message_bytes: Vec<u8>,
    pub crc: u32 
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc_chksm = get_crc_checksum(&chunk_type, &data);
        Chunk {
           data_length: data.len() as u32,
           chunk_type,
           message_bytes: data, 
           crc: crc_chksm 
        }
    }
    pub fn length(&self) -> u32 {
        self.data_length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.message_bytes.as_ref()
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> io::Result<String> {
        Ok(std::str::from_utf8(&self.message_bytes)
            .expect("cannot convert utf8 to str")
            .to_string())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk: Vec<u8> = self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.0.to_be_bytes().iter())
            .chain(self.message_bytes.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect();
        chunk
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let data_length_raw: [u8; 4] = value[0..=3].try_into().expect("failed to get data length");
        let chunk_type_raw: [u8; 4] = value[4..=7].try_into().expect("failed to get chunk_type");
        let crc_raw: [u8; 4] = value[value.len()-4..value.len()].try_into().expect("failed to get chunk_type");
        let mut message_data_raw: Vec<u8> = Vec::new(); 
        if value.len() > 12 {
            for i in value[8..value.len()-4].iter() {
                message_data_raw.push(*i);
            }
        }
        if u32::from_be_bytes(crc_raw) != get_crc_checksum(&ChunkType(u32::from_be_bytes(chunk_type_raw)), &message_data_raw) {
            return Err("crc checksum failed");
        }
        Ok(Chunk { 
            data_length: u32::from_be_bytes(data_length_raw), 
            chunk_type: ChunkType(u32::from_be_bytes(chunk_type_raw)),
            message_bytes: message_data_raw, 
            crc: u32::from_be_bytes(crc_raw) 
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_crc_checksum(chunk_type: &ChunkType, data: &Vec<u8>) -> u32 {
        let maybe_crc = Crc::<u32>::new(&PNG_CRC_ALGO); 
        let mut chunk_type_and_data_bytes: Vec<u8> = Vec::from(u32::to_be_bytes(chunk_type.0));
        for i in data.clone() {
            chunk_type_and_data_bytes.push(i);
        }
        maybe_crc.checksum(&chunk_type_and_data_bytes) 
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
