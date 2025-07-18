// Serialize and Deserialize below

// This is a custom derive macro which will serialize and deserialize the struct as borsh does

// src/macro.rs

// src/serialize_deserialize_macro.rs

use std::fmt;

#[derive(Debug)]
pub struct DeserializeError;

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to deserialize bytes")
    }
}

impl std::error::Error for DeserializeError {}

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize: Sized {
    fn deserialize(v: Vec<u8>) -> Result<Self, DeserializeError>;
}

#[derive(Debug)]
pub struct Swap {
    pub qty_1: u32,
    pub qty_2: u32,
}

impl Serializable for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_le_bytes());
        v.extend_from_slice(&self.qty_2.to_le_bytes());
        v
    }
}

impl Deserialize for Swap {
    fn deserialize(v: Vec<u8>) -> Result<Self, DeserializeError> {
        if v.len() != 8 {
            return Err(DeserializeError);
        }
        let qty_1 = u32::from_le_bytes(v[0..4].try_into().unwrap());
        let qty_2 = u32::from_le_bytes(v[4..8].try_into().unwrap());
        Ok(Swap { qty_1, qty_2 })
    }
}
