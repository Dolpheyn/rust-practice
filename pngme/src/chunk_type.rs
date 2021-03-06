use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    /// Returns true if the 5th bit of the **first** byte in Self::bytes is **0**
    /// (first character in chunk name is uppercase), else return false.
    pub fn is_critical(&self) -> bool {
        self.bytes[0] & 1 << 5 == 0
    }

    /// Returns true if the 5th bit of the **second** byte in Self::bytes is **0**
    /// (second character in chunk name is uppercase), else return false.
    pub fn is_public(&self) -> bool {
        self.bytes[1] & 1 << 5 == 0
    }

    /// Returns true if the 5th bit of the **third** byte in Self::bytes is **0**,
    /// (third character in chunk name is uppercase), else return false.
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] & 1 << 5 == 0
    }

    /// Returns true if the 5th bit of the **fourth** byte in Self::bytes is **not 0**
    /// (fourth character in chunk name is lowercase), else return false.
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] & 1 << 5 != 0
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(&self.bytes)
                .expect("Invalid UTF-8")
                .to_string()
        )
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = s.as_bytes().try_into().expect("str with length 4");

        if !bytes.iter().map(|b| b.is_ascii_alphabetic()).all(|b| b) {
            return Err("ASCII alphabets only bish".into());
        }

        Ok(Self { bytes })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if let true = value.iter().map(u8::is_ascii).all(|v| v) {
            Ok(Self { bytes: value })
        } else {
            Err("Invalid ascii byte found".to_string())
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
