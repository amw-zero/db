use crate::block::Block;
use crate::storage::Storage;
use crate::types::Tuple;
use std::collections::BTreeSet;

// Unit: Number of blocks in the buffer cache
const BUFFER_CACHE_SIZE: usize = 50000;

// Buffers are the in-memory representation of blocks that the
// engine uses for data manipulation. 
#[derive(Copy, Clone)]
struct Buffer {
    block: Block
}

pub struct BufferMgr {
    // The lowest free buffer index
    lowest_free: usize,
    accessed: BTreeSet<usize>,
    buffers: [Buffer; BUFFER_CACHE_SIZE],
}

impl BufferMgr {
    pub fn new() -> Self {
        let accessed = BTreeSet::new();
        BufferMgr {
            lowest_free: 0,
            accessed: accessed,
            buffers: [Buffer { block: Block::new() }; BUFFER_CACHE_SIZE]
        }
    }
}

impl Storage for BufferMgr {    
    fn insert(&mut self, tup: Tuple, table: &str) {
        println!("Inserting tuple: {:?}", tup);
        let buffer = tuple_to_buffer(tup);
        self.buffers[self.lowest_free] = buffer;
        self.accessed.insert(self.lowest_free);
        self.lowest_free += 1;
    }

    fn select(&self, table: &str) -> Vec<Tuple> {
        let mut result = Vec::new();
        for buf in &self.buffers {
            let tup = buffer_to_tuple(buf);
            result.push(tup);
        }

        result
    }
}


fn buffer_to_tuple(buf: &Buffer) -> Tuple {
    // Convert a Block to a Tuple
    vec![] // Placeholder, actual conversion logic needed
}
fn tuple_to_buffer(tup: Tuple) -> Buffer {
    Buffer{block: Block::new()}
}

fn buffer_to_block(buffer: &Buffer) -> Block {
    buffer.block.clone() // Assuming Block implements Clone
}
