use crate::block::{block_decode_value, block_encode_value, Block, BLOCK_SIZE};
use crate::storage::Storage;
use crate::types::Tuple;
use std::collections::BTreeSet;
use std::collections::HashMap;

// Unit: Number of blocks in the buffer cache
const BUFFER_CACHE_SIZE: usize = 5;

// Buffers are the in-memory representation of blocks that the
// engine uses for data manipulation. Buffers contain additional
// metadata useful for the managing of blocks in the buffer cache.
#[derive(Clone)]
struct Buffer {
    size_remaining: usize,
    block: Block,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            size_remaining: BLOCK_SIZE,
            block: Block::new(),
        }
    }
}

struct ItemId {
    // The index of the buffer in the buffer cache
    buffer_idx: usize,

    // The offset within the buffer where the item starts
    offset: usize,

    // Size of the item in bytes
    size: usize,
}

pub struct BufferMgr {
    // Recently accessed buffers, to control eviction
    // Not implemented yet.
    accessed: BTreeSet<usize>,

    // The buffers in the buffer cache
    buffers: Vec<Buffer>,

    // Maps table names to item IDs in the buffer cache
    // to support data retrieval out of buffers
    items: HashMap<String, Vec<ItemId>>,
}

impl BufferMgr {
    pub fn new() -> Self {
        let accessed = BTreeSet::new();
        BufferMgr {
            accessed: accessed,
            buffers: vec![Buffer::new(); BUFFER_CACHE_SIZE],
            items: HashMap::new(),
        }
    }
}

impl Storage for BufferMgr {
    fn insert(&mut self, tup: Tuple, table: &str) {
        let encoded: Vec<Vec<u8>> = tup.into_iter().map(|v| block_encode_value(v)).collect();
        let tuple_size = encoded.iter().map(|v| v.len()).sum::<usize>();
        let mut free_buffer_idx: usize = 0;
        for i in 0..self.buffers.len() - 1 {
            if self.buffers[i].size_remaining >= tuple_size {
                free_buffer_idx = i;
                break;
            }
        }
        let buffer = &mut self.buffers[free_buffer_idx];
        buffer.block.end_ptr += tuple_size;
        buffer.size_remaining -= tuple_size;
        self.items
            .entry(table.to_string())
            .or_insert_with(Vec::new)
            .push(ItemId {
                buffer_idx: free_buffer_idx,
                offset: buffer.block.end_ptr - tuple_size,
                size: tuple_size,
            });
        buffer.block.data[buffer.block.end_ptr - tuple_size..buffer.block.end_ptr]
            .copy_from_slice(&encoded.concat());

        // TODO: Write a test that exposes this condition:
        // if tuple_size > BLOCK_SIZE {
        //     panic!("Tuple size exceeds block size");
        // }
        self.accessed.insert(free_buffer_idx);
    }

    fn select(&self, table: &str) -> Vec<Tuple> {
        let item_ids = self.items.get(table);
        if item_ids.is_none() {
            return vec![];
        }
        let item_ids = item_ids.unwrap();
        let mut result: Vec<Tuple> = vec![];
        for item_id in item_ids {
            let buffer = &self.buffers[item_id.buffer_idx];
            let start = item_id.offset;
            let end = start + item_id.size;
            let data_slice = &buffer.block.data[start..end];
            let value = block_decode_value(data_slice);

            let tuple: Tuple = vec![value]; // Convert the value to a Tuple
            result.push(tuple);
        }

        result
    }
}
