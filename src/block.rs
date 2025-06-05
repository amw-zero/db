// Default block size is 8kb
const BLOCK_SIZE: usize = 8192;

// A block is the unit of storage in our database.
// It is a fixed-size array of bytes that holds tuple data, along
// with metadata. Blocks are the raw data persisted in the database.
// For a more structured representation, see Buffer.
#[derive(Copy, Clone, Debug)]
pub struct Block([u8; BLOCK_SIZE]);

impl Block {
    // Create a new empty block
    pub fn new() -> Self {
        Block([0; BLOCK_SIZE])
    }

    // Get the size of the block
    pub fn size(&self) -> usize {
        BLOCK_SIZE
    }

    // Convert the block to a string representation for debugging
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

