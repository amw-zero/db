use crate::types::Value;

// Default block size is 8kb
pub const BLOCK_SIZE: usize = 8192;

// A block is the unit of storage in our database.
// It is a fixed-size array of bytes that holds tuple data, along
// with metadata. Blocks are the raw data persisted in the database.
// For a more structured representation, see Buffer.
//
// Block data is stored in the heap to prevent stack overflow.
#[derive(Clone, Debug)]
pub struct Block {
    pub end_ptr: usize,
    pub data: Box<[u8; BLOCK_SIZE]>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            end_ptr: 0,
            data: Box::new([0; BLOCK_SIZE]),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn block_encode_value(value: Value) -> Vec<u8> {
    match value {
        Value::Int(i) => i.to_le_bytes().to_vec(),
    }
}

pub fn block_decode_value(encoded: &[u8]) -> Vec<Value> {
    encoded
        .chunks(8)
        .map(|chunk| {
            let bytes = chunk.try_into().expect("Slice with incorrect length");
            Value::Int(i64::from_le_bytes(bytes))
        })
        .collect()
}

pub fn encoded_data_size(encoded_data: &[u8]) -> usize {
    encoded_data.len()
}
